//! Syntax tree of a formula.

use std::collections::BTreeSet;

use super::value::Value;
use super::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Coalesce,
}

impl BinOp {
    pub fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "and",
            BinOp::Or => "or",
            BinOp::Coalesce => "??",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(Value),
    Ident(String),
    List(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Field(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    /// A function from the fixed catalogue.
    Call(String, Vec<Expr>),
    /// `value.name(args)`, e.g. `product(code).attr('kw')`.
    Method(Box<Expr>, String, Vec<Expr>),
    /// `x => body` or `(a, b) => body`; only valid as a function argument.
    Lambda(Vec<String>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    /// The names this formula reads from its surroundings (not lambda parameters, not function
    /// names).
    pub fn free_names(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut bound = Vec::new();
        self.collect(&mut bound, &mut out);
        out
    }

    fn collect(&self, bound: &mut Vec<String>, out: &mut BTreeSet<String>) {
        match &self.kind {
            ExprKind::Lit(_) => {}
            ExprKind::Ident(n) => {
                if !bound.contains(n) {
                    out.insert(n.clone());
                }
            }
            ExprKind::List(items) => items.iter().for_each(|e| e.collect(bound, out)),
            ExprKind::Record(fields) => fields.iter().for_each(|(_, e)| e.collect(bound, out)),
            ExprKind::Unary(_, e) | ExprKind::Field(e, _) => e.collect(bound, out),
            ExprKind::Binary(_, a, b) | ExprKind::Index(a, b) => {
                a.collect(bound, out);
                b.collect(bound, out);
            }
            ExprKind::Ternary(a, b, c) => {
                a.collect(bound, out);
                b.collect(bound, out);
                c.collect(bound, out);
            }
            ExprKind::Call(_, args) => args.iter().for_each(|e| e.collect(bound, out)),
            ExprKind::Method(recv, _, args) => {
                recv.collect(bound, out);
                args.iter().for_each(|e| e.collect(bound, out));
            }
            ExprKind::Lambda(params, body) => {
                let n = bound.len();
                bound.extend(params.iter().cloned());
                body.collect(bound, out);
                bound.truncate(n);
            }
        }
    }

    /// Fields read from a name, e.g. `item.length` gives `("item", "length")`. Used to narrow
    /// dependencies on scope records.
    pub fn field_reads(&self, out: &mut BTreeSet<(String, String)>) {
        match &self.kind {
            ExprKind::Field(inner, f) => {
                if let ExprKind::Ident(n) = &inner.kind {
                    out.insert((n.clone(), f.clone()));
                }
                inner.field_reads(out);
            }
            ExprKind::Lit(_) | ExprKind::Ident(_) => {}
            ExprKind::List(items) => items.iter().for_each(|e| e.field_reads(out)),
            ExprKind::Record(fields) => fields.iter().for_each(|(_, e)| e.field_reads(out)),
            ExprKind::Unary(_, e) => e.field_reads(out),
            ExprKind::Binary(_, a, b) | ExprKind::Index(a, b) => {
                a.field_reads(out);
                b.field_reads(out);
            }
            ExprKind::Ternary(a, b, c) => {
                a.field_reads(out);
                b.field_reads(out);
                c.field_reads(out);
            }
            ExprKind::Call(_, args) => args.iter().for_each(|e| e.field_reads(out)),
            ExprKind::Method(recv, _, args) => {
                recv.field_reads(out);
                args.iter().for_each(|e| e.field_reads(out));
            }
            ExprKind::Lambda(_, body) => body.field_reads(out),
        }
    }
}
