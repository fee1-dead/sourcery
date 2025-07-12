//! Glues lexed tokens to make them more parsable.

use ra_ap_rustc_lexer::TokenKind;
use smol_str::SmolStr;

use crate::Lexer;
use crate::ast::{Braces, Brackets, Delimited, Ident, List, Literal, Parens, Trivia};
use crate::parse::{Punct, TokenStream, TokenTree};

pub struct Gluer<'src> {
    lexer: Lexer<'src>,
}

impl<'src> Gluer<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self { lexer }
    }
    pub fn snapshot(&self) -> Self {
        Gluer { lexer: self.lexer.snapshot() }
    }
    fn peek(&mut self) -> (Trivia, TokenKind, SmolStr) {
        self.lexer.snapshot().next()
    }
    fn peek_nth(&mut self, n: usize) -> (Trivia, TokenKind, SmolStr) {
        let mut l = self.lexer.snapshot();
        let mut ret = l.next();
        for _ in 0..n {
            ret = l.next();
        }
        ret
    }
    pub fn collect_until(&mut self, kind: TokenKind) -> TokenStream {
        let mut stream = TokenStream::default();
        if self.peek().1 == kind {
            return stream;
        }
        let (t1, tt) = self.next();
        stream.t1 = t1;
        stream.tokens = List::single(tt);
        while self.peek().1 != kind {
            let (t, tt) = self.next();
            stream.tokens.push(t, tt);
        }
        stream
    }
    pub fn next(&mut self) -> (Trivia, TokenTree) {
        let (t0, tok, s) = self.lexer.next();
        match tok {
            TokenKind::OpenBrace | TokenKind::OpenParen | TokenKind::OpenBracket => {
                #[rustfmt::skip]
                let (until, delim): (_, fn(_) -> _) = match tok {
                    TokenKind::OpenBrace   => (TokenKind::CloseBrace  , |stream: TokenStream| Delimited::Braces  (Braces  (stream))),
                    TokenKind::OpenParen   => (TokenKind::CloseParen  , |stream: TokenStream| Delimited::Parens  (Parens  (stream))),
                    TokenKind::OpenBracket => (TokenKind::CloseBracket, |stream: TokenStream| Delimited::Brackets(Brackets(stream))),
                    _ => unreachable!(),
                };
                let stream = self.collect_until(until);
                self.lexer.next();
                (t0, TokenTree::Group(Box::new(delim(stream))))
            }
            TokenKind::At => (t0, TokenTree::Punct(Punct::At)),
            TokenKind::Ident => (t0, TokenTree::Ident(Ident(s))),
            TokenKind::RawIdent => (t0, TokenTree::RawIdent(Ident(s))),
            TokenKind::Lifetime {
                starts_with_number: _,
            } => (t0, TokenTree::Lifetime(Ident(s))),
            TokenKind::RawLifetime => (t0, TokenTree::RawLifetime(Ident(s))),

            TokenKind::Literal { kind: _, suffix_start } => {
                let suffix_start = suffix_start as usize;
                let symbol = SmolStr::new(&s[..suffix_start]);
                let suffix = SmolStr::new(&s[suffix_start..]);
                (t0, TokenTree::Literal(Literal { symbol, suffix }))
            }
            
            TokenKind::Pound => {
                (t0, TokenTree::Punct(Punct::Pound))
            }
            TokenKind::Bang => {
                (t0, TokenTree::Punct(Punct::Bang))
            }
            TokenKind::Semi => {
                (t0, TokenTree::Punct(Punct::Semi))
            }
            TokenKind::Colon if matches!(self.peek(), (t, TokenKind::Colon, _) if t.is_empty()) => {
                self.lexer.next();
                (t0, TokenTree::Punct(Punct::ColonColon))
            }
            TokenKind::Eq => {
                (t0, TokenTree::Punct(Punct::Eq))
            }
            TokenKind::Tilde => {
                (t0, TokenTree::Punct(Punct::Tilde))
            }
            TokenKind::Dollar => {
                (t0, TokenTree::Punct(Punct::Dollar))
            }
            TokenKind::Percent => {
                (t0, TokenTree::Punct(Punct::Percent))
            }
            TokenKind::Caret => {
                (t0, TokenTree::Punct(Punct::Caret))
            }
            TokenKind::And => {
                (t0, TokenTree::Punct(Punct::And))
            }
            TokenKind::Star => {
                (t0, TokenTree::Punct(Punct::Star))
            }
            TokenKind::Eof => {
                (t0, TokenTree::Eof)
            }

            TokenKind::CloseBrace | TokenKind::CloseBracket | TokenKind::CloseParen => {
                panic!("unclosed group");
            }
            TokenKind::LineComment { .. }
            | TokenKind::BlockComment { .. }
            | TokenKind::Whitespace { .. } => {
                unreachable!("should be already handled in the lexer")
            }
            TokenKind::InvalidIdent
            | TokenKind::Unknown
            | TokenKind::UnknownPrefix
            | TokenKind::UnknownPrefixLifetime
            | TokenKind::GuardedStrPrefix
            | TokenKind::Frontmatter { .. } => {
                panic!("invalid tokens or weird tokens are unsupported")
            }

            tk => todo!("{tk:?}"),
        }
    }
}
