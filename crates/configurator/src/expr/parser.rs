//! Pratt parser for the formula language.

use super::ast::{BinOp, Expr, ExprKind, UnOp};
use super::lexer::{lex, Tok, Token};
use super::value::Value;
use super::{ExprError, Span};

/// Deepest nesting accepted, so a pathological formula cannot exhaust the stack.
const MAX_DEPTH: usize = 64;
/// Operators applied one after another (`1 + 1 + ...`, `a.b.c...`) nest the tree without nesting
/// the parse; this caps that depth as well, so walking the tree cannot overflow the stack.
const MAX_CHAIN: usize = 4 * MAX_DEPTH;

/// Parses one formula. `offset` is added to every position (template parts).
pub fn parse_at(src: &str, offset: usize) -> Result<Expr, ExprError> {
    let tokens = lex(src, offset)?;
    let mut p = Parser {
        tokens,
        pos: 0,
        depth: 0,
    };
    if p.peek() == &Tok::Eof {
        return Err(ExprError::new("empty formula", p.span()));
    }
    let e = p.expr(0)?;
    if p.peek() != &Tok::Eof {
        return Err(ExprError::new(
            format!("unexpected {}", p.peek().describe()),
            p.span(),
        ));
    }
    check_lambdas(&e, false)?;
    Ok(e)
}

pub fn parse(src: &str) -> Result<Expr, ExprError> {
    parse_at(src, 0)
}

/// Lambdas may only appear directly as function arguments.
fn check_lambdas(e: &Expr, allowed: bool) -> Result<(), ExprError> {
    match &e.kind {
        ExprKind::Lambda(_, body) => {
            if !allowed {
                return Err(ExprError::new(
                    "a function 'x => ...' can only be passed to a function such as filter or map",
                    e.span,
                ));
            }
            check_lambdas(body, false)
        }
        ExprKind::Call(_, args) => args.iter().try_for_each(|a| check_lambdas(a, true)),
        ExprKind::Method(r, _, args) => {
            check_lambdas(r, false)?;
            args.iter().try_for_each(|a| check_lambdas(a, true))
        }
        ExprKind::Lit(_) | ExprKind::Ident(_) => Ok(()),
        ExprKind::List(items) => items.iter().try_for_each(|a| check_lambdas(a, false)),
        ExprKind::Record(f) => f.iter().try_for_each(|(_, a)| check_lambdas(a, false)),
        ExprKind::Unary(_, a) | ExprKind::Field(a, _) => check_lambdas(a, false),
        ExprKind::Binary(_, a, b) | ExprKind::Index(a, b) => {
            check_lambdas(a, false)?;
            check_lambdas(b, false)
        }
        ExprKind::Ternary(a, b, c) => {
            check_lambdas(a, false)?;
            check_lambdas(b, false)?;
            check_lambdas(c, false)
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    depth: usize,
}

fn infix(t: &Tok) -> Option<(u8, u8, Option<BinOp>)> {
    // (left binding power, right binding power, operator); None = ternary
    Some(match t {
        Tok::Question => (2, 1, None),
        Tok::QQ => (4, 3, Some(BinOp::Coalesce)),
        Tok::Or => (5, 6, Some(BinOp::Or)),
        Tok::And => (7, 8, Some(BinOp::And)),
        Tok::EqEq => (9, 10, Some(BinOp::Eq)),
        Tok::NotEq => (9, 10, Some(BinOp::Ne)),
        Tok::Lt => (11, 12, Some(BinOp::Lt)),
        Tok::Le => (11, 12, Some(BinOp::Le)),
        Tok::Gt => (11, 12, Some(BinOp::Gt)),
        Tok::Ge => (11, 12, Some(BinOp::Ge)),
        Tok::Plus => (13, 14, Some(BinOp::Add)),
        Tok::Minus => (13, 14, Some(BinOp::Sub)),
        Tok::Star => (15, 16, Some(BinOp::Mul)),
        Tok::Slash => (15, 16, Some(BinOp::Div)),
        Tok::Percent => (15, 16, Some(BinOp::Rem)),
        _ => return None,
    })
}

const PREFIX_BP: u8 = 17;
const POSTFIX_BP: u8 = 19;

impl Parser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos].tok
    }

    fn peek_at(&self, n: usize) -> &Tok {
        let i = (self.pos + n).min(self.tokens.len() - 1);
        &self.tokens[i].tok
    }

    fn span(&self) -> Span {
        self.tokens[self.pos].span
    }

    fn next(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, t: Tok, what: &str) -> Result<Token, ExprError> {
        if self.peek() == &t {
            Ok(self.next())
        } else {
            Err(ExprError::new(
                format!("expected {what}, found {}", self.peek().describe()),
                self.span(),
            ))
        }
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ExprError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(ExprError::new("formula is nested too deeply", self.span()));
        }
        let r = self.expr_inner(min_bp);
        self.depth -= 1;
        r
    }

    fn expr_inner(&mut self, min_bp: u8) -> Result<Expr, ExprError> {
        let mut lhs = self.prefix()?;
        let mut chain = 0;
        loop {
            let t = self.peek().clone();
            if self.depth + chain > MAX_CHAIN {
                return Err(ExprError::new("formula is nested too deeply", self.span()));
            }
            chain += 1;
            // Postfix: field access, method call, index, call.
            if matches!(t, Tok::Dot | Tok::LBracket | Tok::LParen) {
                if POSTFIX_BP < min_bp {
                    break;
                }
                lhs = self.postfix(lhs)?;
                continue;
            }
            let Some((lbp, rbp, op)) = infix(&t) else {
                break;
            };
            if lbp < min_bp {
                break;
            }
            self.next();
            match op {
                Some(op) => {
                    let rhs = self.expr(rbp)?;
                    let span = lhs.span.join(rhs.span);
                    lhs = Expr {
                        kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
                        span,
                    };
                }
                None => {
                    let then = self.expr(0)?;
                    self.expect(Tok::Colon, "':' of 'condition ? then : else'")?;
                    let otherwise = self.expr(rbp)?;
                    let span = lhs.span.join(otherwise.span);
                    lhs = Expr {
                        kind: ExprKind::Ternary(Box::new(lhs), Box::new(then), Box::new(otherwise)),
                        span,
                    };
                }
            }
        }
        Ok(lhs)
    }

    fn postfix(&mut self, lhs: Expr) -> Result<Expr, ExprError> {
        let t = self.next();
        match t.tok {
            Tok::Dot => {
                let name_tok = self.next();
                let Tok::Ident(name) = name_tok.tok else {
                    return Err(ExprError::new(
                        format!(
                            "expected a name after '.', found {}",
                            name_tok.tok.describe()
                        ),
                        name_tok.span,
                    ));
                };
                if self.peek() == &Tok::LParen {
                    self.next();
                    let (args, end) = self.args()?;
                    let span = lhs.span.join(end);
                    Ok(Expr {
                        kind: ExprKind::Method(Box::new(lhs), name, args),
                        span,
                    })
                } else {
                    let span = lhs.span.join(name_tok.span);
                    Ok(Expr {
                        kind: ExprKind::Field(Box::new(lhs), name),
                        span,
                    })
                }
            }
            Tok::LBracket => {
                let idx = self.expr(0)?;
                let end = self.expect(Tok::RBracket, "']'")?.span;
                let span = lhs.span.join(end);
                Ok(Expr {
                    kind: ExprKind::Index(Box::new(lhs), Box::new(idx)),
                    span,
                })
            }
            Tok::LParen => {
                let ExprKind::Ident(name) = lhs.kind else {
                    return Err(ExprError::new("only functions can be called", lhs.span));
                };
                let (args, end) = self.args()?;
                Ok(Expr {
                    kind: ExprKind::Call(name, args),
                    span: lhs.span.join(end),
                })
            }
            _ => unreachable!(),
        }
    }

    /// Arguments after an opening parenthesis, up to and including the closing one.
    fn args(&mut self) -> Result<(Vec<Expr>, Span), ExprError> {
        let mut args = Vec::new();
        if self.peek() == &Tok::RParen {
            return Ok((args, self.next().span));
        }
        loop {
            args.push(self.expr(0)?);
            match self.peek() {
                Tok::Comma => {
                    self.next();
                }
                Tok::RParen => return Ok((args, self.next().span)),
                other => {
                    return Err(ExprError::new(
                        format!("expected ',' or ')', found {}", other.describe()),
                        self.span(),
                    ))
                }
            }
        }
    }

    fn is_lambda_params(&self) -> bool {
        // At '(': ( ident (, ident)* ) =>   or   ( ) =>
        let mut n = 1;
        if self.peek_at(n) == &Tok::RParen {
            return self.peek_at(n + 1) == &Tok::Arrow;
        }
        loop {
            if !matches!(self.peek_at(n), Tok::Ident(_)) {
                return false;
            }
            n += 1;
            match self.peek_at(n) {
                Tok::Comma => n += 1,
                Tok::RParen => return self.peek_at(n + 1) == &Tok::Arrow,
                _ => return false,
            }
        }
    }

    fn lambda_body(&mut self, params: Vec<String>, start: Span) -> Result<Expr, ExprError> {
        self.expect(Tok::Arrow, "'=>'")?;
        let body = self.expr(1)?;
        let span = start.join(body.span);
        Ok(Expr {
            kind: ExprKind::Lambda(params, Box::new(body)),
            span,
        })
    }

    fn prefix(&mut self) -> Result<Expr, ExprError> {
        let t = self.next();
        let lit = |v: Value| {
            Ok(Expr {
                kind: ExprKind::Lit(v),
                span: t.span,
            })
        };
        match t.tok.clone() {
            Tok::Int(i) => lit(Value::Int(i)),
            Tok::Dec(d) => lit(Value::Dec(d)),
            Tok::Str(s) => lit(Value::Str(s)),
            Tok::True => lit(Value::Bool(true)),
            Tok::False => lit(Value::Bool(false)),
            Tok::Null => lit(Value::Null),
            Tok::Ident(name) => {
                if self.peek() == &Tok::Arrow {
                    return self.lambda_body(vec![name], t.span);
                }
                Ok(Expr {
                    kind: ExprKind::Ident(name),
                    span: t.span,
                })
            }
            Tok::Minus | Tok::Not => {
                let op = if t.tok == Tok::Minus {
                    UnOp::Neg
                } else {
                    UnOp::Not
                };
                let e = self.expr(PREFIX_BP)?;
                let span = t.span.join(e.span);
                Ok(Expr {
                    kind: ExprKind::Unary(op, Box::new(e)),
                    span,
                })
            }
            Tok::LParen => {
                // Step back to look at the whole parenthesised group.
                self.pos -= 1;
                if self.is_lambda_params() {
                    self.next();
                    let mut params = Vec::new();
                    while let Tok::Ident(n) = self.peek().clone() {
                        self.next();
                        params.push(n);
                        if self.peek() == &Tok::Comma {
                            self.next();
                        }
                    }
                    self.expect(Tok::RParen, "')'")?;
                    return self.lambda_body(params, t.span);
                }
                self.next();
                let e = self.expr(0)?;
                let end = self.expect(Tok::RParen, "')'")?.span;
                Ok(Expr {
                    kind: e.kind,
                    span: t.span.join(end),
                })
            }
            Tok::LBracket => {
                let mut items = Vec::new();
                if self.peek() != &Tok::RBracket {
                    loop {
                        items.push(self.expr(0)?);
                        if self.peek() == &Tok::Comma {
                            self.next();
                            if self.peek() == &Tok::RBracket {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                let end = self.expect(Tok::RBracket, "']'")?.span;
                Ok(Expr {
                    kind: ExprKind::List(items),
                    span: t.span.join(end),
                })
            }
            Tok::LBrace => {
                let mut fields: Vec<(String, Expr)> = Vec::new();
                if self.peek() != &Tok::RBrace {
                    loop {
                        let k = self.next();
                        let key = match k.tok {
                            Tok::Ident(s) | Tok::Str(s) => s,
                            other => {
                                return Err(ExprError::new(
                                    format!("expected a key, found {}", other.describe()),
                                    k.span,
                                ))
                            }
                        };
                        if fields.iter().any(|(n, _)| n == &key) {
                            return Err(ExprError::new(
                                format!("key {key} is given twice"),
                                k.span,
                            ));
                        }
                        self.expect(Tok::Colon, "':' after the key")?;
                        let v = self.expr(0)?;
                        fields.push((key, v));
                        if self.peek() == &Tok::Comma {
                            self.next();
                            if self.peek() == &Tok::RBrace {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                let end = self.expect(Tok::RBrace, "'}'")?.span;
                Ok(Expr {
                    kind: ExprKind::Record(fields),
                    span: t.span.join(end),
                })
            }
            other => Err(ExprError::new(
                format!("unexpected {}", other.describe()),
                t.span,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A compact rendering of the tree, to check precedence.
    fn show(e: &Expr) -> String {
        match &e.kind {
            ExprKind::Lit(v) => v.to_text(),
            ExprKind::Ident(n) => n.clone(),
            ExprKind::List(i) => format!("[{}]", i.iter().map(show).collect::<Vec<_>>().join(",")),
            ExprKind::Record(f) => format!(
                "{{{}}}",
                f.iter()
                    .map(|(k, v)| format!("{k}:{}", show(v)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ExprKind::Unary(UnOp::Neg, a) => format!("(-{})", show(a)),
            ExprKind::Unary(UnOp::Not, a) => format!("(not {})", show(a)),
            ExprKind::Binary(op, a, b) => format!("({} {} {})", show(a), op.symbol(), show(b)),
            ExprKind::Ternary(a, b, c) => format!("({} ? {} : {})", show(a), show(b), show(c)),
            ExprKind::Field(a, f) => format!("{}.{f}", show(a)),
            ExprKind::Index(a, i) => format!("{}[{}]", show(a), show(i)),
            ExprKind::Call(n, a) => {
                format!("{n}({})", a.iter().map(show).collect::<Vec<_>>().join(","))
            }
            ExprKind::Method(r, n, a) => format!(
                "{}.{n}({})",
                show(r),
                a.iter().map(show).collect::<Vec<_>>().join(",")
            ),
            ExprKind::Lambda(p, b) => format!("({}) => {}", p.join(","), show(b)),
        }
    }

    fn p(s: &str) -> String {
        show(&parse(s).unwrap_or_else(|e| panic!("{s}: {e:?}")))
    }

    #[test]
    fn precedence_and_associativity() {
        assert_eq!(p("1 + 2 * 3"), "(1 + (2 * 3))");
        assert_eq!(p("(1 + 2) * 3"), "((1 + 2) * 3)");
        assert_eq!(p("a - b - c"), "((a - b) - c)");
        assert_eq!(p("-a.b"), "(-a.b)");
        assert_eq!(p("not a and b or c"), "(((not a) and b) or c)");
        assert_eq!(p("a < 1 == b >= 2"), "((a < 1) == (b >= 2))");
        assert_eq!(p("a ?? b ?? c"), "(a ?? (b ?? c))");
        assert_eq!(p("a ? b : c ? d : e"), "(a ? b : (c ? d : e))");
        assert_eq!(p("x > 1 ? 'a' : 'b'"), "((x > 1) ? a : b)");
        assert_eq!(p("a ?? 1 + 2"), "(a ?? (1 + 2))");
        assert_eq!(p("a and b ? 1 : 2"), "((a and b) ? 1 : 2)");
    }

    #[test]
    fn postfix_calls_lambdas_and_literals() {
        assert_eq!(p("product(code).attr('kw')"), "product(code).attr(kw)");
        assert_eq!(
            p("lookup('m', {p: 3, kw: x}, 1).size"),
            "lookup(m,{p:3,kw:x},1).size"
        );
        assert_eq!(p("map(xs, x => x * 2)"), "map(xs,(x) => (x * 2))");
        assert_eq!(p("sum(xs, (a) => a.n)"), "sum(xs,(a) => a.n)");
        assert_eq!(
            p("reduce(xs, (a, b) => a + b)"),
            "reduce(xs,(a,b) => (a + b))"
        );
        assert_eq!(p("[1, 2, 3,][0]"), "[1,2,3][0]");
        assert_eq!(
            p("first([1,2,4], n => n > 1)"),
            "first([1,2,4],(n) => (n > 1))"
        );
        assert_eq!(p("(a)"), "a");
    }

    #[test]
    fn errors_carry_positions() {
        let e = parse("1 +").unwrap_err();
        assert_eq!(e.start, 3, "{e:?}");
        let e = parse("a ? b").unwrap_err();
        assert!(e.message.contains("':'"), "{e:?}");
        assert_eq!(e.start, 5);
        let e = parse("f(1 2)").unwrap_err();
        assert_eq!((e.start, e.end), (4, 5));
        let e = parse("x => x").unwrap_err();
        assert!(e.message.contains("filter"), "{e:?}");
        let e = parse("(1)(2)").unwrap_err();
        assert!(e.message.contains("only functions"));
        let e = parse("").unwrap_err();
        assert!(e.message.contains("empty"));
        let e = parse("{a: 1, a: 2}").unwrap_err();
        assert_eq!(e.start, 7);
        let deep = "(".repeat(100) + "1" + &")".repeat(100);
        assert!(parse(&deep).unwrap_err().message.contains("deeply"));
        // A long chain of operators nests the tree as deeply, without parentheses.
        let long = vec!["1"; 10_000].join(" + ");
        assert!(parse(&long).unwrap_err().message.contains("deeply"));
        let fields = format!("a{}", ".b".repeat(10_000));
        assert!(parse(&fields).unwrap_err().message.contains("deeply"));
        assert!(parse(&vec!["1"; 100].join(" + ")).is_ok());
        let e = parse_at("a +", 10).unwrap_err();
        assert_eq!(e.start, 13);
    }
}
