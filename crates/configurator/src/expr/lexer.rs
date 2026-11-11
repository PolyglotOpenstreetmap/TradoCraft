//! Tokens of the formula language. Positions are character offsets into the source.

use rust_decimal::Decimal;

use super::{ExprError, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Int(i64),
    Dec(Decimal),
    Str(String),
    Ident(String),
    True,
    False,
    Null,
    And,
    Or,
    Not,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    Question,
    QQ,
    Colon,
    Dot,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Arrow,
    Eof,
}

impl Tok {
    pub fn describe(&self) -> String {
        match self {
            Tok::Int(i) => i.to_string(),
            Tok::Dec(d) => d.to_string(),
            Tok::Str(s) => format!("'{s}'"),
            Tok::Ident(s) => s.clone(),
            Tok::True => "true".into(),
            Tok::False => "false".into(),
            Tok::Null => "null".into(),
            Tok::And => "and".into(),
            Tok::Or => "or".into(),
            Tok::Not => "not".into(),
            Tok::Plus => "+".into(),
            Tok::Minus => "-".into(),
            Tok::Star => "*".into(),
            Tok::Slash => "/".into(),
            Tok::Percent => "%".into(),
            Tok::EqEq => "==".into(),
            Tok::NotEq => "!=".into(),
            Tok::Lt => "<".into(),
            Tok::Le => "<=".into(),
            Tok::Gt => ">".into(),
            Tok::Ge => ">=".into(),
            Tok::Question => "?".into(),
            Tok::QQ => "??".into(),
            Tok::Colon => ":".into(),
            Tok::Dot => ".".into(),
            Tok::Comma => ",".into(),
            Tok::LParen => "(".into(),
            Tok::RParen => ")".into(),
            Tok::LBracket => "[".into(),
            Tok::RBracket => "]".into(),
            Tok::LBrace => "{".into(),
            Tok::RBrace => "}".into(),
            Tok::Arrow => "=>".into(),
            Tok::Eof => "end of formula".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
}

/// Splits `src` into tokens; `offset` is added to every position (used for template parts).
pub fn lex(src: &str, offset: usize) -> Result<Vec<Token>, ExprError> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let span = |a: usize, b: usize| Span::new(a + offset, b + offset);
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        if c.is_ascii_digit() {
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_') {
                i += 1;
            }
            let mut is_dec = false;
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1].is_ascii_digit() {
                is_dec = true;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_') {
                    i += 1;
                }
            }
            let text: String = chars[start..i].iter().filter(|c| **c != '_').collect();
            let tok = if is_dec {
                Tok::Dec(text.parse().map_err(|_| {
                    ExprError::new(format!("invalid number {text}"), span(start, i))
                })?)
            } else {
                match text.parse::<i64>() {
                    Ok(v) => Tok::Int(v),
                    Err(_) => Tok::Dec(text.parse().map_err(|_| {
                        ExprError::new(format!("number {text} is too large"), span(start, i))
                    })?),
                }
            };
            out.push(Token {
                tok,
                span: span(start, i),
            });
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let tok = match word.as_str() {
                "true" => Tok::True,
                "false" => Tok::False,
                "null" => Tok::Null,
                "and" => Tok::And,
                "or" => Tok::Or,
                "not" => Tok::Not,
                _ => Tok::Ident(word),
            };
            out.push(Token {
                tok,
                span: span(start, i),
            });
            continue;
        }
        if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            let mut s = String::new();
            loop {
                if i >= chars.len() {
                    return Err(ExprError::new("unterminated text", span(start, i)));
                }
                let ch = chars[i];
                if ch == quote {
                    i += 1;
                    break;
                }
                if ch == '\\' && i + 1 < chars.len() {
                    let n = chars[i + 1];
                    s.push(match n {
                        'n' => '\n',
                        't' => '\t',
                        other => other,
                    });
                    i += 2;
                    continue;
                }
                s.push(ch);
                i += 1;
            }
            out.push(Token {
                tok: Tok::Str(s),
                span: span(start, i),
            });
            continue;
        }
        let next = chars.get(i + 1).copied();
        let (tok, len) = match (c, next) {
            ('=', Some('=')) => (Tok::EqEq, 2),
            ('=', Some('>')) => (Tok::Arrow, 2),
            ('!', Some('=')) => (Tok::NotEq, 2),
            ('<', Some('=')) => (Tok::Le, 2),
            ('>', Some('=')) => (Tok::Ge, 2),
            ('?', Some('?')) => (Tok::QQ, 2),
            ('&', Some('&')) => (Tok::And, 2),
            ('|', Some('|')) => (Tok::Or, 2),
            ('!', _) => (Tok::Not, 1),
            ('<', _) => (Tok::Lt, 1),
            ('>', _) => (Tok::Gt, 1),
            ('?', _) => (Tok::Question, 1),
            ('+', _) => (Tok::Plus, 1),
            ('-', _) => (Tok::Minus, 1),
            ('*', _) => (Tok::Star, 1),
            ('/', _) => (Tok::Slash, 1),
            ('%', _) => (Tok::Percent, 1),
            (':', _) => (Tok::Colon, 1),
            ('.', _) => (Tok::Dot, 1),
            (',', _) => (Tok::Comma, 1),
            ('(', _) => (Tok::LParen, 1),
            (')', _) => (Tok::RParen, 1),
            ('[', _) => (Tok::LBracket, 1),
            (']', _) => (Tok::RBracket, 1),
            ('{', _) => (Tok::LBrace, 1),
            ('}', _) => (Tok::RBrace, 1),
            ('=', _) => {
                return Err(ExprError::new(
                    "unexpected '='; use '==' to compare",
                    span(start, start + 1),
                ))
            }
            _ => {
                return Err(ExprError::new(
                    format!("unexpected character '{c}'"),
                    span(start, start + 1),
                ))
            }
        };
        i += len;
        out.push(Token {
            tok,
            span: span(start, i),
        });
    }
    out.push(Token {
        tok: Tok::Eof,
        span: span(chars.len(), chars.len()),
    });
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(s: &str) -> Vec<Tok> {
        lex(s, 0).unwrap().into_iter().map(|t| t.tok).collect()
    }

    #[test]
    fn lexes_numbers_strings_and_operators() {
        assert_eq!(
            toks("a.b >= 1.5 ?? 'x' && !c"),
            [
                Tok::Ident("a".into()),
                Tok::Dot,
                Tok::Ident("b".into()),
                Tok::Ge,
                Tok::Dec("1.5".parse().unwrap()),
                Tok::QQ,
                Tok::Str("x".into()),
                Tok::And,
                Tok::Not,
                Tok::Ident("c".into()),
                Tok::Eof
            ]
        );
        assert_eq!(toks("1_000 x=>y")[0], Tok::Int(1000));
        assert_eq!(toks("1_000 x=>y")[2], Tok::Arrow);
        assert_eq!(toks("1.")[0], Tok::Int(1));
    }

    #[test]
    fn reports_positions() {
        let e = lex("a + #", 0).unwrap_err();
        assert_eq!((e.start, e.end), (4, 5));
        let e = lex("'open", 3).unwrap_err();
        assert_eq!(e.start, 3);
        assert!(lex("a = b", 0).unwrap_err().message.contains("=="));
    }
}
