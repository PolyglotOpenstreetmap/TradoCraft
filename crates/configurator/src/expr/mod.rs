//! The formula language of configurators (ADR 0034).
//!
//! A small, safe expression language: no I/O, no loops other than list functions, a step budget
//! per evaluation and exact decimal arithmetic. Pipeline: [`lexer`] → [`parser`] (Pratt, with
//! positions) → [`types`] (static checks) → [`interp`] (evaluation). Templates are text with
//! `{formula}` parts.

pub mod ast;
pub mod functions;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod value;

use serde::Serialize;
use utoipa::ToSchema;

pub use ast::Expr;
pub use interp::{eval, eval_with, Budget, Env, Host, NoHost};
pub use parser::{parse, parse_at};
pub use types::{check, check_with, OpenEnv, TableInfo, Type, TypeEnv};
pub use value::Value;

/// A range of characters in the source (0-based, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn join(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

/// A syntax, type or evaluation error with its place in the formula.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct ExprError {
    pub message: String,
    /// Character offset where the problem starts (0-based).
    pub start: usize,
    /// Character offset just after the problem.
    pub end: usize,
}

impl ExprError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        ExprError {
            message: message.into(),
            start: span.start,
            end: span.end.max(span.start),
        }
    }
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at {})", self.message, self.start + 1)
    }
}

/// A part of a template.
#[derive(Debug, Clone)]
pub enum Part {
    Text(String),
    Expr(Expr),
}

/// Text with `{formula}` parts, e.g. `DRV-{motor.size}{phases == 3 ? 'T' : 'M'}`.
/// `{{` and `}}` stand for literal braces.
#[derive(Debug, Clone)]
pub struct Template {
    pub parts: Vec<Part>,
}

impl Template {
    pub fn parse(src: &str) -> Result<Template, ExprError> {
        let chars: Vec<char> = src.chars().collect();
        let mut parts = Vec::new();
        let mut text = String::new();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '{' && chars.get(i + 1) == Some(&'{') {
                text.push('{');
                i += 2;
                continue;
            }
            if c == '}' && chars.get(i + 1) == Some(&'}') {
                text.push('}');
                i += 2;
                continue;
            }
            if c == '}' {
                return Err(ExprError::new(
                    "unmatched '}' (write '}}' for a brace)",
                    Span::new(i, i + 1),
                ));
            }
            if c != '{' {
                text.push(c);
                i += 1;
                continue;
            }
            // Find the matching '}', skipping quoted text and nested braces.
            let start = i + 1;
            let mut depth = 0usize;
            let mut j = start;
            let mut quote: Option<char> = None;
            let end = loop {
                if j >= chars.len() {
                    return Err(ExprError::new("unclosed '{'", Span::new(i, i + 1)));
                }
                let ch = chars[j];
                match quote {
                    Some(q) => {
                        if ch == '\\' {
                            j += 1;
                        } else if ch == q {
                            quote = None;
                        }
                    }
                    None => match ch {
                        '\'' | '"' => quote = Some(ch),
                        '{' => depth += 1,
                        '}' if depth == 0 => break j,
                        '}' => depth -= 1,
                        _ => {}
                    },
                }
                j += 1;
            };
            if !text.is_empty() {
                parts.push(Part::Text(std::mem::take(&mut text)));
            }
            let inner: String = chars[start..end].iter().collect();
            parts.push(Part::Expr(parse_at(&inner, start)?));
            i = end + 1;
        }
        if !text.is_empty() {
            parts.push(Part::Text(text));
        }
        Ok(Template { parts })
    }

    pub fn exprs(&self) -> impl Iterator<Item = &Expr> {
        self.parts.iter().filter_map(|p| match p {
            Part::Expr(e) => Some(e),
            Part::Text(_) => None,
        })
    }

    pub fn check(&self, locals: &[(String, Type)], env: &dyn TypeEnv) -> Vec<ExprError> {
        self.exprs()
            .flat_map(|e| check_with(e, locals, env).1)
            .collect()
    }

    pub fn render(
        &self,
        locals: &[(String, Value)],
        env: &dyn Env,
        host: &dyn Host,
        budget: &Budget,
    ) -> Result<String, ExprError> {
        let mut out = String::new();
        for p in &self.parts {
            match p {
                Part::Text(t) => out.push_str(t),
                Part::Expr(e) => out.push_str(&eval_with(e, locals, env, host, budget)?.to_text()),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests;
