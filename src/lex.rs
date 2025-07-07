use crate::{Trivia, Trivium};

use ra_ap_rustc_lexer as rustc_lexer;

use rustc_lexer::{Cursor, TokenKind};
use smol_str::SmolStr;

pub struct Lexer<'src> {
    pub orig_str: &'src str,
    pub inner: Cursor<'src>,
    pub cur_pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn next(&mut self) -> (Trivia, TokenKind, SmolStr) {
        use TokenKind::*;
        let mut trivia = Trivia { list: vec![] };

        loop {
            let start = self.cur_pos;
            let tok = self.inner.advance_token();
            self.cur_pos += tok.len as usize;
            let snippet = SmolStr::new(&self.orig_str[start..self.cur_pos]);
            break match tok.kind {
                Whitespace => {
                    trivia.list.push(Trivium::Whitespace(snippet));
                    continue;
                }
                LineComment { doc_style: _ } => {
                    trivia.list.push(Trivium::LineComment(snippet));
                    continue;
                }
                BlockComment {
                    terminated: _,
                    doc_style: _,
                } => {
                    trivia.list.push(Trivium::BlockComment(snippet));
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
                | Eof => (trivia, tok.kind, snippet),
            };
        }
    }
}

pub fn tokenize(s: &str) -> Lexer<'_> {
    Lexer {
        orig_str: s,
        inner: Cursor::new(s, rustc_lexer::FrontmatterAllowed::Yes),
        cur_pos: 0,
    }
}
