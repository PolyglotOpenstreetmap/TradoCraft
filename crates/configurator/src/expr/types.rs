//! Static type checker. It infers a type for every formula and reports definite mistakes (unknown
//! names, functions, tables and columns, wrong argument counts, arithmetic on text, ...). Values
//! whose type is not known statically are `any` and checked at run time.

use std::collections::BTreeMap;
use std::fmt;

use super::ast::{BinOp, Expr, ExprKind, UnOp};
use super::functions::{lambda_positions, signature};
use super::value::Value;
use super::ExprError;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Null,
    Bool,
    Int,
    Dec,
    Str,
    List(Box<Type>),
    /// A record with known fields, or `None` when the fields are not known.
    Record(Option<BTreeMap<String, Type>>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Any => write!(f, "any"),
            Type::Null => write!(f, "null"),
            Type::Bool => write!(f, "boolean"),
            Type::Int => write!(f, "integer"),
            Type::Dec => write!(f, "decimal"),
            Type::Str => write!(f, "text"),
            Type::List(t) => write!(f, "list of {t}"),
            Type::Record(_) => write!(f, "record"),
        }
    }
}

impl Type {
    pub fn of_value(v: &Value) -> Type {
        match v {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Dec(_) => Type::Dec,
            Value::Str(_) => Type::Str,
            Value::List(items) => Type::List(Box::new(
                items
                    .iter()
                    .map(Type::of_value)
                    .reduce(|a, b| join(&a, &b))
                    .unwrap_or(Type::Any),
            )),
            Value::Record(m) => Type::Record(Some(
                m.iter()
                    .map(|(k, v)| (k.clone(), Type::of_value(v)))
                    .collect(),
            )),
        }
    }

    fn numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Dec | Type::Any | Type::Null)
    }

    fn boolish(&self) -> bool {
        matches!(self, Type::Bool | Type::Any | Type::Null)
    }

    fn listish(&self) -> bool {
        matches!(self, Type::List(_) | Type::Any | Type::Null)
    }

    fn elem(&self) -> Type {
        match self {
            Type::List(t) => (**t).clone(),
            _ => Type::Any,
        }
    }
}

/// The common type of two branches.
pub fn join(a: &Type, b: &Type) -> Type {
    match (a, b) {
        _ if a == b => a.clone(),
        (Type::Null, x) | (x, Type::Null) => x.clone(),
        (Type::Int, Type::Dec) | (Type::Dec, Type::Int) => Type::Dec,
        (Type::List(x), Type::List(y)) => Type::List(Box::new(join(x, y))),
        (Type::Record(_), Type::Record(_)) => Type::Record(None),
        _ => Type::Any,
    }
}

/// A table as the checker sees it.
#[derive(Debug, Clone, Default)]
pub struct TableInfo {
    pub columns: BTreeMap<String, Type>,
    /// Keys `lookup` accepts (exact key columns plus the range key).
    pub keys: Vec<String>,
}

/// Names and tables a formula can see.
pub trait TypeEnv {
    /// The type of a name; `None` if unknown.
    fn name(&self, name: &str) -> Option<Type>;
    fn table(&self, name: &str) -> Option<TableInfo>;
    /// When false, unknown names and tables are not errors (checking a lone formula).
    fn closed(&self) -> bool {
        true
    }
}

/// An environment that knows nothing and accepts every name as `any`.
pub struct OpenEnv;

impl TypeEnv for OpenEnv {
    fn name(&self, _: &str) -> Option<Type> {
        None
    }
    fn table(&self, _: &str) -> Option<TableInfo> {
        None
    }
    fn closed(&self) -> bool {
        false
    }
}

pub fn product_type() -> Type {
    let mut m = BTreeMap::new();
    m.insert("code".into(), Type::Str);
    m.insert("name".into(), Type::Str);
    m.insert("family".into(), Type::Str);
    m.insert("unit".into(), Type::Str);
    m.insert("active".into(), Type::Bool);
    m.insert("attributes".into(), Type::Record(None));
    Type::Record(Some(m))
}

/// Checks a formula: its inferred type and every error found.
pub fn check(e: &Expr, env: &dyn TypeEnv) -> (Type, Vec<ExprError>) {
    let mut c = Checker {
        env,
        locals: Vec::new(),
        errors: Vec::new(),
    };
    let t = c.check(e);
    (t, c.errors)
}

/// Like [`check`] with extra local names (e.g. `each`, `parent`).
pub fn check_with(
    e: &Expr,
    locals: &[(String, Type)],
    env: &dyn TypeEnv,
) -> (Type, Vec<ExprError>) {
    let mut c = Checker {
        env,
        locals: locals.to_vec(),
        errors: Vec::new(),
    };
    let t = c.check(e);
    (t, c.errors)
}

struct Checker<'a> {
    env: &'a dyn TypeEnv,
    locals: Vec<(String, Type)>,
    errors: Vec<ExprError>,
}

impl Checker<'_> {
    fn error(&mut self, msg: impl Into<String>, e: &Expr) {
        self.errors.push(ExprError::new(msg, e.span));
    }

    fn check(&mut self, e: &Expr) -> Type {
        match &e.kind {
            ExprKind::Lit(v) => Type::of_value(v),
            ExprKind::Ident(n) => {
                if let Some((_, t)) = self.locals.iter().rev().find(|(x, _)| x == n) {
                    return t.clone();
                }
                if let Some(t) = self.env.name(n) {
                    return t;
                }
                if n == "pi" {
                    return Type::Dec;
                }
                if signature(n).is_some() {
                    self.error(format!("{n} is a function; write {n}(...)"), e);
                } else if self.env.closed() {
                    self.error(format!("unknown name {n}"), e);
                }
                Type::Any
            }
            ExprKind::List(items) => {
                let t = items
                    .iter()
                    .map(|i| self.check(i))
                    .reduce(|a, b| join(&a, &b))
                    .unwrap_or(Type::Any);
                Type::List(Box::new(t))
            }
            ExprKind::Record(fields) => Type::Record(Some(
                fields
                    .iter()
                    .map(|(k, v)| (k.clone(), self.check(v)))
                    .collect(),
            )),
            ExprKind::Unary(op, a) => {
                let t = self.check(a);
                match op {
                    UnOp::Neg => {
                        if !t.numeric() {
                            self.error(format!("cannot negate {t}"), a);
                            return Type::Any;
                        }
                        t
                    }
                    UnOp::Not => {
                        if !t.boolish() {
                            self.error(format!("'not' needs a yes/no value, found {t}"), a);
                        }
                        Type::Bool
                    }
                }
            }
            ExprKind::Binary(op, a, b) => {
                let ta = self.check(a);
                let tb = self.check(b);
                self.binary(*op, &ta, &tb, a, b, e)
            }
            ExprKind::Ternary(c, a, b) => {
                let tc = self.check(c);
                if !tc.boolish() {
                    self.error(
                        format!("the condition must be a yes/no value, found {tc}"),
                        c,
                    );
                }
                let ta = self.check(a);
                let tb = self.check(b);
                join(&ta, &tb)
            }
            ExprKind::Field(inner, f) => {
                let t = self.check(inner);
                match t {
                    Type::Record(Some(fields)) => match fields.get(f) {
                        Some(t) => t.clone(),
                        None => {
                            let known: Vec<&str> = fields.keys().map(String::as_str).collect();
                            self.error(format!("no field {f} (known: {})", known.join(", ")), e);
                            Type::Any
                        }
                    },
                    Type::Record(None) | Type::Any | Type::Null => Type::Any,
                    other => {
                        self.error(format!("cannot read .{f} of {other}"), e);
                        Type::Any
                    }
                }
            }
            ExprKind::Index(inner, idx) => {
                let t = self.check(inner);
                let ti = self.check(idx);
                match t {
                    Type::List(el) => {
                        if !matches!(ti, Type::Int | Type::Any | Type::Null) {
                            self.error(
                                format!("a list index must be a whole number, found {ti}"),
                                idx,
                            );
                        }
                        *el
                    }
                    Type::Record(_) | Type::Any | Type::Null => Type::Any,
                    other => {
                        self.error(format!("cannot index {other}"), e);
                        Type::Any
                    }
                }
            }
            ExprKind::Method(recv, name, args) => {
                let t = self.check(recv);
                for a in args {
                    self.check(a);
                }
                match name.as_str() {
                    "attr" | "get" => {
                        if args.len() != 1 {
                            self.error(format!("{name}(key) takes one argument"), e);
                        }
                        if !matches!(t, Type::Record(_) | Type::Any | Type::Null) {
                            self.error(format!("cannot call .{name} on {t}"), e);
                        }
                        Type::Any
                    }
                    _ => {
                        self.error(format!("unknown method .{name}()"), e);
                        Type::Any
                    }
                }
            }
            ExprKind::Lambda(_, body) => {
                self.error("a function 'x => ...' can only be passed to a function", e);
                self.check(body)
            }
            ExprKind::Call(name, args) => self.call(name, args, e),
        }
    }

    fn binary(&mut self, op: BinOp, ta: &Type, tb: &Type, a: &Expr, b: &Expr, e: &Expr) -> Type {
        match op {
            BinOp::And | BinOp::Or => {
                for (t, x) in [(ta, a), (tb, b)] {
                    if !t.boolish() {
                        self.error(
                            format!("'{}' needs yes/no values, found {t}", op.symbol()),
                            x,
                        );
                    }
                }
                Type::Bool
            }
            BinOp::Coalesce => join(ta, tb),
            BinOp::Eq | BinOp::Ne => Type::Bool,
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                let ok = (ta.numeric() && tb.numeric())
                    || matches!(
                        (ta, tb),
                        (
                            Type::Str | Type::Any | Type::Null,
                            Type::Str | Type::Any | Type::Null
                        )
                    );
                if !ok {
                    self.error(format!("cannot compare {ta} with {tb}"), e);
                }
                Type::Bool
            }
            _ => {
                if op == BinOp::Add && matches!((ta, tb), (Type::Str, Type::Str)) {
                    return Type::Str;
                }
                if !ta.numeric() || !tb.numeric() {
                    self.error(format!("cannot use {} on {ta} and {tb}", op.symbol()), e);
                    return Type::Any;
                }
                match (ta, tb) {
                    (Type::Any, _) | (_, Type::Any) => Type::Any,
                    _ if op == BinOp::Div => Type::Dec,
                    (Type::Int | Type::Null, Type::Int | Type::Null) => Type::Int,
                    _ => Type::Dec,
                }
            }
        }
    }

    fn lambda(&mut self, f: &Expr, params: &[Type]) -> Type {
        let ExprKind::Lambda(names, body) = &f.kind else {
            self.check(f);
            self.error("expected a function such as x => x.value", f);
            return Type::Any;
        };
        if names.len() > params.len() {
            self.error(
                format!("this function takes {} parameter(s)", params.len()),
                f,
            );
        }
        let n = self.locals.len();
        for (name, t) in names.iter().zip(params) {
            self.locals.push((name.clone(), t.clone()));
        }
        let t = self.check(body);
        self.locals.truncate(n);
        t
    }

    fn table_arg(&mut self, a: &Expr) -> Option<TableInfo> {
        match &a.kind {
            ExprKind::Lit(Value::Str(name)) => match self.env.table(name) {
                Some(t) => Some(t),
                None => {
                    if self.env.closed() {
                        self.error(format!("unknown table {name}"), a);
                    }
                    None
                }
            },
            _ => {
                let t = self.check(a);
                if !matches!(t, Type::Str | Type::Any) {
                    self.error(format!("the table name must be text, found {t}"), a);
                }
                None
            }
        }
    }

    fn call(&mut self, name: &str, args: &[Expr], e: &Expr) -> Type {
        let Some(sig) = signature(name) else {
            self.error(format!("unknown function {name}()"), e);
            for a in args {
                if !matches!(a.kind, ExprKind::Lambda(..)) {
                    self.check(a);
                }
            }
            return Type::Any;
        };
        if args.len() < sig.min || args.len() > sig.max {
            let want = if sig.min == sig.max {
                format!("{}", sig.min)
            } else if sig.max >= 64 {
                format!("at least {}", sig.min)
            } else {
                format!("{} to {}", sig.min, sig.max)
            };
            self.error(
                format!("{name}() takes {want} argument(s), not {}", args.len()),
                e,
            );
            return Type::Any;
        }
        let lambdas = lambda_positions(name);
        // Special forms first.
        match name {
            "lookup" | "rows" => {
                let info = self.table_arg(&args[0]);
                let row = Type::Record(info.as_ref().map(|i| i.columns.clone()));
                if name == "rows" {
                    return Type::List(Box::new(row));
                }
                let tk = self.check(&args[1]);
                match (&args[1].kind, &info) {
                    (ExprKind::Record(fields), Some(i)) => {
                        for (k, _) in fields {
                            if !i.keys.contains(k) {
                                self.error(
                                    format!(
                                        "{k} is not a key of this table (keys: {})",
                                        i.keys.join(", ")
                                    ),
                                    &args[1],
                                );
                            }
                        }
                    }
                    _ => {
                        if !matches!(tk, Type::Record(_) | Type::Any) {
                            self.error(
                                format!("the keys must be a record like {{phases: 3}}, found {tk}"),
                                &args[1],
                            );
                        }
                    }
                }
                if let Some(o) = args.get(2) {
                    let t = self.check(o);
                    if !matches!(t, Type::Int | Type::Any | Type::Null) {
                        self.error(format!("the offset must be a whole number, found {t}"), o);
                    }
                }
                return row;
            }
            "min" | "max"
                if args.len() == 1
                    || (args.len() == 2 && matches!(args[1].kind, ExprKind::Lambda(..))) =>
            {
                let t = self.check(&args[0]);
                return match (&t, args.get(1)) {
                    (Type::List(el), Some(f)) => self.lambda(f, &[(**el).clone()]),
                    (Type::List(el), None) => (**el).clone(),
                    (Type::Any | Type::Null, Some(f)) => self.lambda(f, &[Type::Any]),
                    (_, Some(f)) => {
                        self.error(format!("expected a list, found {t}"), &args[0]);
                        self.lambda(f, &[Type::Any])
                    }
                    (other, None) => other.clone(),
                };
            }
            _ => {}
        }
        // Lambda-taking functions: the first argument is the list.
        if !lambdas.is_empty() {
            let tl = self.check(&args[0]);
            if !tl.listish() {
                self.error(format!("{name}() needs a list, found {tl}"), &args[0]);
            }
            let el = tl.elem();
            let mapped = args
                .get(1)
                .map(|f| self.lambda(f, std::slice::from_ref(&el)));
            if matches!(name, "count" | "any" | "all" | "filter" | "first") {
                if let Some(t) = &mapped {
                    if !t.boolish() {
                        self.error(
                            format!("the condition must be a yes/no value, found {t}"),
                            &args[1],
                        );
                    }
                }
            }
            let m = mapped.clone().unwrap_or_else(|| el.clone());
            return match name {
                "sum" => {
                    if !m.numeric() {
                        self.error(format!("sum() needs numbers, found {m}"), e);
                    }
                    match m {
                        Type::Int => Type::Int,
                        Type::Any => Type::Any,
                        _ => Type::Dec,
                    }
                }
                "avg" => Type::Dec,
                "count" => Type::Int,
                "any" | "all" => Type::Bool,
                "distinct" | "map" => Type::List(Box::new(m)),
                "filter" | "sort" => Type::List(Box::new(el)),
                "first" => el,
                _ => Type::Any,
            };
        }
        let types: Vec<Type> = args.iter().map(|a| self.check(a)).collect();
        let need_num = |c: &mut Self, i: usize| {
            if !types[i].numeric() {
                c.error(
                    format!("{name}() needs a number, found {}", types[i]),
                    &args[i],
                );
            }
        };
        match name {
            "ceil" | "floor" | "round" => {
                need_num(self, 0);
                if args.len() == 2 {
                    need_num(self, 1);
                    let zero = matches!(args[1].kind, ExprKind::Lit(Value::Int(0)));
                    if !zero {
                        return Type::Dec;
                    }
                }
                Type::Int
            }
            "abs" => {
                need_num(self, 0);
                types[0].clone()
            }
            "sqrt" | "sin" | "cos" => {
                need_num(self, 0);
                Type::Dec
            }
            "pow" => {
                need_num(self, 0);
                need_num(self, 1);
                Type::Dec
            }
            "pi" => Type::Dec,
            "min" | "max" => types
                .iter()
                .cloned()
                .reduce(|a, b| join(&a, &b))
                .unwrap_or(Type::Any),
            "concat" | "str" | "upper" | "lower" => Type::Str,
            "capture" => {
                self.check_pattern(&args[1]);
                Type::Str
            }
            "pad" => {
                need_num(self, 1);
                Type::Str
            }
            "number" => Type::Dec,
            "len" => Type::Int,
            "contains" | "matches" | "exists" => {
                if name == "matches" {
                    self.check_pattern(&args[1]);
                }
                Type::Bool
            }
            "last" => {
                if !types[0].listish() {
                    self.error(format!("last() needs a list, found {}", types[0]), &args[0]);
                }
                types[0].elem()
            }
            "range" | "chunks" => {
                need_num(self, 0);
                need_num(self, 1);
                Type::List(Box::new(Type::Int))
            }
            "bins" => {
                if !types[0].listish() {
                    self.error(format!("bins() needs a list, found {}", types[0]), &args[0]);
                }
                need_num(self, 1);
                Type::List(Box::new(Type::List(Box::new(types[0].elem()))))
            }
            "product" => product_type(),
            _ => Type::Any,
        }
    }

    /// A literal pattern is compiled once here, so a typo is reported while editing.
    fn check_pattern(&mut self, a: &Expr) {
        if let ExprKind::Lit(Value::Str(p)) = &a.kind {
            if let Err(e) = regex::RegexBuilder::new(p).size_limit(1 << 20).build() {
                self.error(format!("invalid pattern: {e}"), a);
            }
        }
    }
}
