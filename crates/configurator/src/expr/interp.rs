//! Interpreter: evaluates a checked or unchecked tree with exact decimal arithmetic.
//!
//! Safety: there is no I/O, every visited node costs one step of a shared [`Budget`], lists are
//! capped at [`MAX_LIST`] elements and regular expressions are size limited.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap};

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, RoundingStrategy};

use super::ast::{BinOp, Expr, ExprKind, UnOp};
use super::value::{parse_decimal, Value};
use super::{ExprError, Span};

/// Longest list a formula may build.
pub const MAX_LIST: usize = 10_000;

/// Steps shared by all formulas of one evaluation.
#[derive(Debug)]
pub struct Budget {
    used: Cell<u64>,
    limit: u64,
}

impl Budget {
    pub fn new(limit: u64) -> Self {
        Budget {
            used: Cell::new(0),
            limit,
        }
    }

    pub fn used(&self) -> u64 {
        self.used.get()
    }

    fn step(&self, span: Span) -> Result<(), ExprError> {
        let n = self.used.get() + 1;
        self.used.set(n);
        if n > self.limit {
            return Err(ExprError::new(
                format!("step budget of {} exceeded", self.limit),
                span,
            ));
        }
        Ok(())
    }
}

/// Names visible to a formula (fields, computed values, scope records).
pub trait Env {
    fn get(&self, name: &str) -> Option<Value>;
}

impl Env for BTreeMap<String, Value> {
    fn get(&self, name: &str) -> Option<Value> {
        BTreeMap::get(self, name).cloned()
    }
}

/// What a formula can ask its host: tables and the product catalogue.
pub trait Host {
    /// A table row as a record, or null; `Err` for an unknown table or a lookup refused by the
    /// table's out-of-range policy.
    fn lookup(
        &self,
        table: &str,
        keys: &BTreeMap<String, Value>,
        offset: i64,
    ) -> Result<Value, String>;
    /// All rows of a table as a list of records.
    fn rows(&self, table: &str) -> Result<Value, String>;
    /// The product with this code as a record (`code`, `name`, `family`, `active`, `unit`,
    /// `attributes`), or null if unknown.
    fn product(&self, code: &str) -> Value;
}

/// A host without tables or products (for checks of single formulas).
pub struct NoHost;

impl Host for NoHost {
    fn lookup(&self, table: &str, _: &BTreeMap<String, Value>, _: i64) -> Result<Value, String> {
        Err(format!("unknown table {table}"))
    }
    fn rows(&self, table: &str) -> Result<Value, String> {
        Err(format!("unknown table {table}"))
    }
    fn product(&self, _: &str) -> Value {
        Value::Null
    }
}

pub fn eval(e: &Expr, env: &dyn Env, host: &dyn Host, budget: &Budget) -> Result<Value, ExprError> {
    let mut it = Interp {
        env,
        host,
        budget,
        locals: Vec::new(),
        regexes: RefCell::new(HashMap::new()),
    };
    it.eval(e)
}

/// Evaluates with extra local names bound first (e.g. `each`, `parent`).
pub fn eval_with(
    e: &Expr,
    locals: &[(String, Value)],
    env: &dyn Env,
    host: &dyn Host,
    budget: &Budget,
) -> Result<Value, ExprError> {
    let mut it = Interp {
        env,
        host,
        budget,
        locals: locals.to_vec(),
        regexes: RefCell::new(HashMap::new()),
    };
    it.eval(e)
}

struct Interp<'a> {
    env: &'a dyn Env,
    host: &'a dyn Host,
    budget: &'a Budget,
    locals: Vec<(String, Value)>,
    regexes: RefCell<HashMap<String, regex::Regex>>,
}

fn err<T>(msg: impl Into<String>, span: Span) -> Result<T, ExprError> {
    Err(ExprError::new(msg, span))
}

fn int_or_dec(d: Decimal) -> Value {
    Value::Dec(d)
}

/// Truthiness of a condition: booleans, with null counting as false.
fn truthy(v: &Value, span: Span) -> Result<bool, ExprError> {
    match v {
        Value::Bool(b) => Ok(*b),
        Value::Null => Ok(false),
        other => err(
            format!("expected a yes/no value, found {}", other.type_name()),
            span,
        ),
    }
}

fn to_f64(d: Decimal) -> f64 {
    d.to_f64().unwrap_or(0.0)
}

fn from_f64(f: f64, span: Span) -> Result<Value, ExprError> {
    if !f.is_finite() {
        return err("result is not a number", span);
    }
    Decimal::try_from(f)
        .map(|d| Value::Dec(d.round_dp(12).normalize()))
        .map_err(|_| ExprError::new("result is out of range", span))
}

fn check_len(n: usize, span: Span) -> Result<(), ExprError> {
    if n > MAX_LIST {
        return err(format!("a list may have at most {MAX_LIST} elements"), span);
    }
    Ok(())
}

impl<'a> Interp<'a> {
    fn lookup_name(&self, name: &str, span: Span) -> Result<Value, ExprError> {
        if let Some((_, v)) = self.locals.iter().rev().find(|(n, _)| n == name) {
            return Ok(v.clone());
        }
        if let Some(v) = self.env.get(name) {
            return Ok(v);
        }
        if name == "pi" {
            return Ok(Value::Dec(Decimal::PI));
        }
        err(format!("unknown name {name}"), span)
    }

    fn eval(&mut self, e: &Expr) -> Result<Value, ExprError> {
        self.budget.step(e.span)?;
        match &e.kind {
            ExprKind::Lit(v) => Ok(v.clone()),
            ExprKind::Ident(n) => self.lookup_name(n, e.span),
            ExprKind::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for i in items {
                    out.push(self.eval(i)?);
                }
                Ok(Value::List(out))
            }
            ExprKind::Record(fields) => {
                let mut out = BTreeMap::new();
                for (k, v) in fields {
                    let v = self.eval(v)?;
                    out.insert(k.clone(), v);
                }
                Ok(Value::Record(out))
            }
            ExprKind::Unary(op, a) => {
                let v = self.eval(a)?;
                match op {
                    UnOp::Neg => match v {
                        Value::Null => Ok(Value::Null),
                        Value::Int(i) => Ok(i
                            .checked_neg()
                            .map(Value::Int)
                            .unwrap_or_else(|| Value::Dec(-Decimal::from(i)))),
                        Value::Dec(d) => Ok(Value::Dec(-d)),
                        other => err(format!("cannot negate {}", other.type_name()), e.span),
                    },
                    UnOp::Not => Ok(Value::Bool(!truthy(&v, a.span)?)),
                }
            }
            ExprKind::Binary(op, a, b) => self.binary(*op, a, b, e.span),
            ExprKind::Ternary(c, a, b) => {
                let cv = self.eval(c)?;
                if truthy(&cv, c.span)? {
                    self.eval(a)
                } else {
                    self.eval(b)
                }
            }
            ExprKind::Field(inner, f) => {
                let v = self.eval(inner)?;
                match v {
                    Value::Null => Ok(Value::Null),
                    Value::Record(mut m) => match m.remove(f) {
                        Some(x) => Ok(x),
                        None => err(format!("the record has no field {f}"), e.span),
                    },
                    other => err(format!("cannot read .{f} of {}", other.type_name()), e.span),
                }
            }
            ExprKind::Index(inner, idx) => {
                let v = self.eval(inner)?;
                let i = self.eval(idx)?;
                match (v, i) {
                    (Value::Null, _) | (_, Value::Null) => Ok(Value::Null),
                    (Value::List(items), i) => {
                        let Some(i) = i.as_int() else {
                            return err("a list index must be a whole number", idx.span);
                        };
                        // Negative indexes count from the end.
                        let n = items.len() as i64;
                        let k = if i < 0 { n + i } else { i };
                        Ok(if k >= 0 && k < n {
                            items[k as usize].clone()
                        } else {
                            Value::Null
                        })
                    }
                    (Value::Record(m), Value::Str(k)) => {
                        Ok(m.get(&k).cloned().unwrap_or(Value::Null))
                    }
                    (other, _) => err(format!("cannot index {}", other.type_name()), e.span),
                }
            }
            ExprKind::Call(name, args) => self.call(name, args, e.span),
            ExprKind::Method(recv, name, args) => {
                let r = self.eval(recv)?;
                match name.as_str() {
                    "attr" | "get" => {
                        if args.len() != 1 {
                            return err(format!("{name}(key) takes one argument"), e.span);
                        }
                        let k = self.eval(&args[0])?;
                        let Value::Str(k) = k else {
                            return err("the key must be text", args[0].span);
                        };
                        match r {
                            Value::Null => Ok(Value::Null),
                            Value::Record(m) => {
                                let src = if name == "attr" {
                                    match m.get("attributes") {
                                        Some(Value::Record(a)) => a,
                                        _ => &m,
                                    }
                                } else {
                                    &m
                                };
                                Ok(src.get(&k).cloned().unwrap_or(Value::Null))
                            }
                            other => err(
                                format!("cannot call .{name} on {}", other.type_name()),
                                e.span,
                            ),
                        }
                    }
                    _ => err(format!("unknown method .{name}()"), e.span),
                }
            }
            ExprKind::Lambda(..) => err(
                "a function 'x => ...' can only be passed to a function",
                e.span,
            ),
        }
    }

    fn binary(&mut self, op: BinOp, a: &Expr, b: &Expr, span: Span) -> Result<Value, ExprError> {
        match op {
            BinOp::And => {
                let l = self.eval(a)?;
                if !truthy(&l, a.span)? {
                    return Ok(Value::Bool(false));
                }
                let r = self.eval(b)?;
                return Ok(Value::Bool(truthy(&r, b.span)?));
            }
            BinOp::Or => {
                let l = self.eval(a)?;
                if truthy(&l, a.span)? {
                    return Ok(Value::Bool(true));
                }
                let r = self.eval(b)?;
                return Ok(Value::Bool(truthy(&r, b.span)?));
            }
            BinOp::Coalesce => {
                let l = self.eval(a)?;
                return if l.is_null() { self.eval(b) } else { Ok(l) };
            }
            _ => {}
        }
        let l = self.eval(a)?;
        let r = self.eval(b)?;
        match op {
            BinOp::Eq => return Ok(Value::Bool(l.loose_eq(&r))),
            BinOp::Ne => return Ok(Value::Bool(!l.loose_eq(&r))),
            _ => {}
        }
        if l.is_null() || r.is_null() {
            return Ok(Value::Null);
        }
        match op {
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                let ord = match (&l, &r) {
                    (Value::Str(x), Value::Str(y)) => x.cmp(y),
                    _ => match (l.as_decimal(), r.as_decimal()) {
                        (Some(x), Some(y)) => x.cmp(&y),
                        _ => {
                            return err(
                                format!("cannot compare {} with {}", l.type_name(), r.type_name()),
                                span,
                            )
                        }
                    },
                };
                use std::cmp::Ordering::*;
                Ok(Value::Bool(match op {
                    BinOp::Lt => ord == Less,
                    BinOp::Le => ord != Greater,
                    BinOp::Gt => ord == Greater,
                    _ => ord != Less,
                }))
            }
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                if op == BinOp::Add {
                    if let (Value::Str(x), Value::Str(y)) = (&l, &r) {
                        return Ok(Value::Str(format!("{x}{y}")));
                    }
                }
                if let (Value::Int(x), Value::Int(y)) = (&l, &r) {
                    let (x, y) = (*x, *y);
                    let res = match op {
                        BinOp::Add => x.checked_add(y),
                        BinOp::Sub => x.checked_sub(y),
                        BinOp::Mul => x.checked_mul(y),
                        BinOp::Rem => {
                            if y == 0 {
                                return err("division by zero", span);
                            }
                            x.checked_rem(y)
                        }
                        _ => None,
                    };
                    if let Some(v) = res {
                        return Ok(Value::Int(v));
                    }
                }
                let (Some(x), Some(y)) = (l.as_decimal(), r.as_decimal()) else {
                    return err(
                        format!(
                            "cannot use {} on {} and {}",
                            op.symbol(),
                            l.type_name(),
                            r.type_name()
                        ),
                        span,
                    );
                };
                let res = match op {
                    BinOp::Add => x.checked_add(y),
                    BinOp::Sub => x.checked_sub(y),
                    BinOp::Mul => x.checked_mul(y),
                    BinOp::Div => {
                        if y.is_zero() {
                            return err("division by zero", span);
                        }
                        x.checked_div(y)
                    }
                    _ => {
                        if y.is_zero() {
                            return err("division by zero", span);
                        }
                        x.checked_rem(y)
                    }
                };
                res.map(int_or_dec)
                    .ok_or_else(|| ExprError::new("number out of range", span))
            }
            _ => unreachable!(),
        }
    }

    fn call_lambda(&mut self, f: &Expr, args: Vec<Value>) -> Result<Value, ExprError> {
        let ExprKind::Lambda(params, body) = &f.kind else {
            return err("expected a function such as x => x.value", f.span);
        };
        if params.len() > args.len() {
            return err(
                format!("this function takes {} parameter(s)", args.len()),
                f.span,
            );
        }
        let n = self.locals.len();
        for (p, a) in params.iter().zip(args) {
            self.locals.push((p.clone(), a));
        }
        let r = self.eval(body);
        self.locals.truncate(n);
        r
    }

    fn list_arg(&mut self, e: &Expr) -> Result<Option<Vec<Value>>, ExprError> {
        match self.eval(e)? {
            Value::List(l) => Ok(Some(l)),
            Value::Null => Ok(None),
            other => err(
                format!("expected a list, found {}", other.type_name()),
                e.span,
            ),
        }
    }

    /// The list of a `fn(list, x => ...)` call, mapped through the function when given.
    fn mapped(&mut self, args: &[Expr]) -> Result<Option<Vec<Value>>, ExprError> {
        let Some(list) = self.list_arg(&args[0])? else {
            return Ok(None);
        };
        match args.get(1) {
            None => Ok(Some(list)),
            Some(f) => {
                let mut out = Vec::with_capacity(list.len());
                for v in list {
                    out.push(self.call_lambda(f, vec![v])?);
                }
                Ok(Some(out))
            }
        }
    }

    fn num_arg(&mut self, e: &Expr) -> Result<Option<Decimal>, ExprError> {
        match self.eval(e)? {
            Value::Null => Ok(None),
            v => match v.as_decimal() {
                Some(d) => Ok(Some(d)),
                None => err(
                    format!("expected a number, found {}", v.type_name()),
                    e.span,
                ),
            },
        }
    }

    fn int_arg(&mut self, e: &Expr) -> Result<Option<i64>, ExprError> {
        match self.eval(e)? {
            Value::Null => Ok(None),
            v => match v.as_int() {
                Some(d) => Ok(Some(d)),
                None => err(
                    format!("expected a whole number, found {}", v.type_name()),
                    e.span,
                ),
            },
        }
    }

    fn str_arg(&mut self, e: &Expr) -> Result<Option<String>, ExprError> {
        match self.eval(e)? {
            Value::Null => Ok(None),
            Value::Str(s) => Ok(Some(s)),
            v => Ok(Some(v.to_text())),
        }
    }

    fn regex(&self, pattern: &str, span: Span) -> Result<regex::Regex, ExprError> {
        if let Some(r) = self.regexes.borrow().get(pattern) {
            return Ok(r.clone());
        }
        let r = regex::RegexBuilder::new(pattern)
            .size_limit(1 << 20)
            .dfa_size_limit(1 << 20)
            .build()
            .map_err(|e| ExprError::new(format!("invalid pattern: {e}"), span))?;
        self.regexes
            .borrow_mut()
            .insert(pattern.to_string(), r.clone());
        Ok(r)
    }

    fn arity(
        name: &str,
        args: &[Expr],
        min: usize,
        max: usize,
        span: Span,
    ) -> Result<(), ExprError> {
        if args.len() < min || args.len() > max {
            let want = if min == max {
                format!("{min}")
            } else {
                format!("{min} to {max}")
            };
            return err(
                format!("{name}() takes {want} argument(s), not {}", args.len()),
                span,
            );
        }
        Ok(())
    }

    fn call(&mut self, name: &str, args: &[Expr], span: Span) -> Result<Value, ExprError> {
        let Some(sig) = super::functions::signature(name) else {
            return err(format!("unknown function {name}()"), span);
        };
        Self::arity(name, args, sig.min, sig.max, span)?;
        match name {
            "ceil" | "floor" | "round" => {
                let Some(x) = self.num_arg(&args[0])? else {
                    return Ok(Value::Null);
                };
                let dp = match args.get(1) {
                    Some(a) => self.int_arg(a)?.unwrap_or(0),
                    None => 0,
                };
                if !(0..=12).contains(&dp) {
                    return err("digits must be between 0 and 12", span);
                }
                let strategy = match name {
                    "ceil" => RoundingStrategy::ToPositiveInfinity,
                    "floor" => RoundingStrategy::ToNegativeInfinity,
                    _ => RoundingStrategy::MidpointAwayFromZero,
                };
                let r = x.round_dp_with_strategy(dp as u32, strategy);
                if dp == 0 {
                    if let Some(i) = r.to_i64() {
                        return Ok(Value::Int(i));
                    }
                }
                Ok(Value::Dec(r))
            }
            "abs" => match self.eval(&args[0])? {
                Value::Null => Ok(Value::Null),
                Value::Int(i) => Ok(i
                    .checked_abs()
                    .map(Value::Int)
                    .unwrap_or_else(|| Value::Dec(Decimal::from(i).abs()))),
                Value::Dec(d) => Ok(Value::Dec(d.abs())),
                v => err(
                    format!("expected a number, found {}", v.type_name()),
                    args[0].span,
                ),
            },
            "sqrt" | "sin" | "cos" => {
                let Some(x) = self.num_arg(&args[0])? else {
                    return Ok(Value::Null);
                };
                let f = to_f64(x);
                if name == "sqrt" && f < 0.0 {
                    return err("square root of a negative number", span);
                }
                from_f64(
                    match name {
                        "sqrt" => f.sqrt(),
                        "sin" => f.sin(),
                        _ => f.cos(),
                    },
                    span,
                )
            }
            "pow" => {
                let (Some(x), Some(n)) = (self.num_arg(&args[0])?, self.int_arg(&args[1])?) else {
                    return Ok(Value::Null);
                };
                if !(0..=64).contains(&n) {
                    return err("the exponent must be between 0 and 64", args[1].span);
                }
                let mut r = Decimal::ONE;
                for _ in 0..n {
                    r = r
                        .checked_mul(x)
                        .ok_or_else(|| ExprError::new("number out of range", span))?;
                }
                Ok(Value::Dec(r))
            }
            "pi" => Ok(Value::Dec(Decimal::PI)),
            "min" | "max" => {
                // min(list), min(list, x => ...), or min(a, b, ...)
                let values = if args.len() == 1
                    || (args.len() == 2 && matches!(args[1].kind, ExprKind::Lambda(..)))
                {
                    let first = self.eval(&args[0])?;
                    if let Value::List(_) | Value::Null = first {
                        match self.mapped(args)? {
                            Some(v) => v,
                            None => return Ok(Value::Null),
                        }
                    } else {
                        vec![first]
                    }
                } else {
                    let mut v = Vec::new();
                    for a in args {
                        v.push(self.eval(a)?);
                    }
                    v
                };
                let mut best: Option<Value> = None;
                for v in values {
                    if v.is_null() {
                        continue;
                    }
                    let better = match &best {
                        None => true,
                        Some(b) => {
                            let ord = match (&v, b) {
                                (Value::Str(x), Value::Str(y)) => x.cmp(y),
                                _ => match (v.as_decimal(), b.as_decimal()) {
                                    (Some(x), Some(y)) => x.cmp(&y),
                                    _ => {
                                        return err(
                                            format!("{name}() needs numbers or texts"),
                                            span,
                                        )
                                    }
                                },
                            };
                            if name == "min" {
                                ord.is_lt()
                            } else {
                                ord.is_gt()
                            }
                        }
                    };
                    if better {
                        best = Some(v);
                    }
                }
                Ok(best.unwrap_or(Value::Null))
            }
            "concat" => {
                let mut s = String::new();
                for a in args {
                    s.push_str(&self.eval(a)?.to_text());
                }
                Ok(Value::Str(s))
            }
            "str" => Ok(Value::Str(self.eval(&args[0])?.to_text())),
            "number" => match self.eval(&args[0])? {
                Value::Null => Ok(Value::Null),
                v @ (Value::Int(_) | Value::Dec(_)) => Ok(v),
                Value::Str(s) => Ok(parse_decimal(&s).map(Value::Dec).unwrap_or(Value::Null)),
                v => err(
                    format!("cannot convert {} to a number", v.type_name()),
                    span,
                ),
            },
            "pad" => {
                let Some(s) = self.str_arg(&args[0])? else {
                    return Ok(Value::Null);
                };
                let width = self.int_arg(&args[1])?.unwrap_or(0).clamp(0, 1000) as usize;
                let fill = match args.get(2) {
                    Some(a) => self.str_arg(a)?.unwrap_or_else(|| "0".into()),
                    None => "0".into(),
                };
                let Some(fill) = fill.chars().next() else {
                    return err("the fill character is empty", span);
                };
                let n = s.chars().count();
                let mut out: String = std::iter::repeat_n(fill, width.saturating_sub(n)).collect();
                out.push_str(&s);
                Ok(Value::Str(out))
            }
            "upper" | "lower" => Ok(match self.str_arg(&args[0])? {
                None => Value::Null,
                Some(s) => Value::Str(if name == "upper" {
                    s.to_uppercase()
                } else {
                    s.to_lowercase()
                }),
            }),
            "len" => match self.eval(&args[0])? {
                Value::Null => Ok(Value::Int(0)),
                Value::Str(s) => Ok(Value::Int(s.chars().count() as i64)),
                Value::List(l) => Ok(Value::Int(l.len() as i64)),
                v => err(format!("len() of {}", v.type_name()), span),
            },
            "contains" => match self.eval(&args[0])? {
                Value::Null => Ok(Value::Bool(false)),
                Value::Str(s) => {
                    let n = self.str_arg(&args[1])?.unwrap_or_default();
                    Ok(Value::Bool(s.contains(&n)))
                }
                Value::List(l) => {
                    let n = self.eval(&args[1])?;
                    Ok(Value::Bool(l.iter().any(|x| x.loose_eq(&n))))
                }
                v => err(format!("contains() on {}", v.type_name()), span),
            },
            "matches" => {
                let (Some(s), Some(p)) = (self.str_arg(&args[0])?, self.str_arg(&args[1])?) else {
                    return Ok(Value::Bool(false));
                };
                let re = self.regex(&p, args[1].span)?;
                Ok(Value::Bool(re.is_match(&s)))
            }
            "capture" => {
                let (Some(s), Some(p)) = (self.str_arg(&args[0])?, self.str_arg(&args[1])?) else {
                    return Ok(Value::Null);
                };
                let group = match args.get(2) {
                    Some(a) => self.int_arg(a)?.unwrap_or(1),
                    None => 1,
                };
                let re = self.regex(&p, args[1].span)?;
                if group < 0 || group as usize >= re.captures_len() {
                    return err(
                        format!("the pattern has no group {group}"),
                        args.get(2).map(|a| a.span).unwrap_or(span),
                    );
                }
                Ok(re
                    .captures(&s)
                    .and_then(|c| c.get(group as usize))
                    .map(|m| Value::Str(m.as_str().to_string()))
                    .unwrap_or(Value::Null))
            }
            "sum" | "avg" => {
                let Some(values) = self.mapped(args)? else {
                    return Ok(if name == "sum" {
                        Value::Int(0)
                    } else {
                        Value::Null
                    });
                };
                let mut all_int = true;
                let mut total = Decimal::ZERO;
                let mut n = 0i64;
                for v in &values {
                    match v {
                        Value::Null => continue,
                        Value::Int(i) => {
                            total = total
                                .checked_add(Decimal::from(*i))
                                .ok_or_else(|| ExprError::new("number out of range", span))?
                        }
                        Value::Dec(d) => {
                            all_int = false;
                            total = total
                                .checked_add(*d)
                                .ok_or_else(|| ExprError::new("number out of range", span))?;
                        }
                        other => {
                            return err(
                                format!("{name}() needs numbers, found {}", other.type_name()),
                                span,
                            )
                        }
                    }
                    n += 1;
                }
                if name == "avg" {
                    return Ok(if n == 0 {
                        Value::Null
                    } else {
                        Value::Dec(total / Decimal::from(n))
                    });
                }
                Ok(if all_int {
                    total.to_i64().map(Value::Int).unwrap_or(Value::Dec(total))
                } else {
                    Value::Dec(total)
                })
            }
            "count" => {
                let Some(list) = self.list_arg(&args[0])? else {
                    return Ok(Value::Int(0));
                };
                match args.get(1) {
                    None => Ok(Value::Int(list.len() as i64)),
                    Some(f) => {
                        let mut n = 0;
                        for v in list {
                            let r = self.call_lambda(f, vec![v])?;
                            if truthy(&r, f.span)? {
                                n += 1;
                            }
                        }
                        Ok(Value::Int(n))
                    }
                }
            }
            "any" | "all" => {
                let Some(list) = self.list_arg(&args[0])? else {
                    return Ok(Value::Bool(name == "all"));
                };
                for v in list {
                    let r = match args.get(1) {
                        Some(f) => {
                            let r = self.call_lambda(f, vec![v])?;
                            truthy(&r, f.span)?
                        }
                        None => truthy(&v, args[0].span)?,
                    };
                    if name == "any" && r {
                        return Ok(Value::Bool(true));
                    }
                    if name == "all" && !r {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(name == "all"))
            }
            "distinct" => {
                let Some(values) = self.mapped(args)? else {
                    return Ok(Value::List(vec![]));
                };
                let mut out: Vec<Value> = Vec::new();
                for v in values {
                    if !out.iter().any(|x| x.loose_eq(&v)) {
                        out.push(v);
                    }
                }
                Ok(Value::List(out))
            }
            "filter" => {
                let Some(list) = self.list_arg(&args[0])? else {
                    return Ok(Value::List(vec![]));
                };
                let mut out = Vec::new();
                for v in list {
                    let r = self.call_lambda(&args[1], vec![v.clone()])?;
                    if truthy(&r, args[1].span)? {
                        out.push(v);
                    }
                }
                Ok(Value::List(out))
            }
            "map" => Ok(Value::List(self.mapped(args)?.unwrap_or_default())),
            "first" => {
                let Some(list) = self.list_arg(&args[0])? else {
                    return Ok(Value::Null);
                };
                match args.get(1) {
                    None => Ok(list.into_iter().next().unwrap_or(Value::Null)),
                    Some(f) => {
                        for v in list {
                            let r = self.call_lambda(f, vec![v.clone()])?;
                            if truthy(&r, f.span)? {
                                return Ok(v);
                            }
                        }
                        Ok(Value::Null)
                    }
                }
            }
            "last" => Ok(self
                .list_arg(&args[0])?
                .and_then(|l| l.into_iter().last())
                .unwrap_or(Value::Null)),
            "sort" => {
                let Some(list) = self.list_arg(&args[0])? else {
                    return Ok(Value::List(vec![]));
                };
                let mut keyed = Vec::with_capacity(list.len());
                for v in list {
                    let k = match args.get(1) {
                        Some(f) => self.call_lambda(f, vec![v.clone()])?,
                        None => v.clone(),
                    };
                    keyed.push((k, v));
                }
                let mut bad = false;
                keyed.sort_by(|(a, _), (b, _)| match (a, b) {
                    (Value::Str(x), Value::Str(y)) => x.cmp(y),
                    (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
                    (Value::Null, _) => std::cmp::Ordering::Greater,
                    (_, Value::Null) => std::cmp::Ordering::Less,
                    _ => match (a.as_decimal(), b.as_decimal()) {
                        (Some(x), Some(y)) => x.cmp(&y),
                        _ => {
                            bad = true;
                            std::cmp::Ordering::Equal
                        }
                    },
                });
                if bad {
                    return err("sort() needs numbers or texts to sort by", span);
                }
                Ok(Value::List(keyed.into_iter().map(|(_, v)| v).collect()))
            }
            "range" => {
                let (Some(a), Some(b)) = (self.int_arg(&args[0])?, self.int_arg(&args[1])?) else {
                    return Ok(Value::List(vec![]));
                };
                if b < a {
                    return Ok(Value::List(vec![]));
                }
                check_len((b - a + 1) as usize, span)?;
                Ok(Value::List((a..=b).map(Value::Int).collect()))
            }
            "chunks" => {
                let (Some(n), Some(size)) = (self.int_arg(&args[0])?, self.int_arg(&args[1])?)
                else {
                    return Ok(Value::List(vec![]));
                };
                if size <= 0 {
                    return err("the chunk size must be at least 1", args[1].span);
                }
                if n <= 0 {
                    return Ok(Value::List(vec![]));
                }
                let full = n / size;
                let rest = n % size;
                check_len((full + i64::from(rest > 0)) as usize, span)?;
                let mut out: Vec<Value> = (0..full).map(|_| Value::Int(size)).collect();
                if rest > 0 {
                    out.push(Value::Int(rest));
                }
                Ok(Value::List(out))
            }
            "bins" => {
                // First fit, in the given order: each bin is the list of values it holds.
                let Some(values) = self.list_arg(&args[0])? else {
                    return Ok(Value::List(vec![]));
                };
                let Some(cap) = self.num_arg(&args[1])? else {
                    return Ok(Value::List(vec![]));
                };
                if cap <= Decimal::ZERO {
                    return err("the capacity must be positive", args[1].span);
                }
                let mut bins: Vec<(Decimal, Vec<Value>)> = Vec::new();
                for v in values {
                    let Some(d) = v.as_decimal() else {
                        if v.is_null() {
                            continue;
                        }
                        return err("bins() needs numbers", args[0].span);
                    };
                    if d > cap {
                        return err(
                            format!(
                                "{} does not fit in a bin of {}",
                                d.normalize(),
                                cap.normalize()
                            ),
                            span,
                        );
                    }
                    match bins.iter_mut().find(|(used, _)| *used + d <= cap) {
                        Some((used, items)) => {
                            *used += d;
                            items.push(v);
                        }
                        None => bins.push((d, vec![v])),
                    }
                }
                Ok(Value::List(
                    bins.into_iter()
                        .map(|(_, items)| Value::List(items))
                        .collect(),
                ))
            }
            "lookup" => {
                let table = match self.eval(&args[0])? {
                    Value::Str(s) => s,
                    v => {
                        return err(
                            format!("the table name must be text, found {}", v.type_name()),
                            args[0].span,
                        )
                    }
                };
                let keys = match self.eval(&args[1])? {
                    Value::Record(m) => m,
                    v => {
                        return err(
                            format!(
                                "the keys must be a record like {{phases: 3}}, found {}",
                                v.type_name()
                            ),
                            args[1].span,
                        )
                    }
                };
                let offset = match args.get(2) {
                    Some(a) => self.int_arg(a)?.unwrap_or(0),
                    None => 0,
                };
                self.host
                    .lookup(&table, &keys, offset)
                    .map_err(|m| ExprError::new(m, span))
            }
            "rows" => {
                let table = match self.eval(&args[0])? {
                    Value::Str(s) => s,
                    v => {
                        return err(
                            format!("the table name must be text, found {}", v.type_name()),
                            args[0].span,
                        )
                    }
                };
                self.host.rows(&table).map_err(|m| ExprError::new(m, span))
            }
            "product" => Ok(match self.str_arg(&args[0])? {
                None => Value::Null,
                Some(code) => self.host.product(code.trim()),
            }),
            "exists" => Ok(Value::Bool(match self.str_arg(&args[0])? {
                None => false,
                Some(code) => match self.host.product(code.trim()) {
                    Value::Record(m) => m.get("active") == Some(&Value::Bool(true)),
                    _ => false,
                },
            })),
            _ => err(format!("unknown function {name}()"), span),
        }
    }
}
