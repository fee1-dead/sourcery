use std::ops::Range;

use crate::{Trivia, Trivium, TriviumKind};

use ra_ap_rustc_lexer as rustc_lexer;

use rustc_lexer::{Cursor, TokenKind};

pub struct Lexer<'src> {
    pub inner: Cursor<'src>,
    pub cur_pos: u32,
}

impl<'src> Lexer<'src> {
    pub fn next(&mut self) -> (Trivia, TokenKind, Range<u32>) {
        use TokenKind::*;
        let mut trivia = Trivia { list: vec![] };

        loop {
            let start = self.cur_pos;
            let tok = self.inner.advance_token();
            self.cur_pos += tok.len;
            let span = start..self.cur_pos;
            break match tok.kind {
                Whitespace => {
                    trivia.list.push(Trivium {
                        kind: TriviumKind::Whitespace,
                        span,
                    });
                    continue;
                }
                LineComment { doc_style: _ } => {
                    trivia.list.push(Trivium {
                        kind: TriviumKind::LineComment,
                        span,
                    });
                    continue;
                }
                BlockComment {
                    terminated: _,
                    doc_style: _,
                } => {
                    trivia.list.push(Trivium {
                        kind: TriviumKind::LineComment,
                        span,
                    });
                    continue;
                }
                Frontmatter { .. }
                | Ident
                | InvalidIdent
                | RawIdent
                | UnknownPrefix
                | UnknownPrefixLifetime
                | RawLifetime
                | GuardedStrPrefix
                | Literal { .. }
                | Lifetime { .. }
                | Semi
                | Comma
                | Dot
                | OpenParen
                | CloseParen
                | OpenBrace
                | CloseBrace
                | OpenBracket
                | CloseBracket
                | At
                | Pound
                | Tilde
                | Question
                | Colon
                | Dollar
                | Eq
                | Bang
                | Lt
                | Gt
                | Minus
                | And
                | Or
                | Plus
                | Star
                | Slash
                | Caret
                | Percent
                | Unknown
                | Eof => (trivia, tok.kind, span),
            };
        }
    }
}

pub fn tokenize(s: &str) -> Lexer<'_> {
    Lexer {
        inner: Cursor::new(s, rustc_lexer::FrontmatterAllowed::Yes),
        cur_pos: 0,
    }
}
