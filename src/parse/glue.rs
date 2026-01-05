//! Glues lexed tokens to make them more parsable.

use ra_ap_rustc_lexer::TokenKind;
use smol_str::SmolStr;

use crate::Lexer;
use crate::prelude::*;

pub struct Gluer<'src> {
    lexer: Lexer<'src>,
}

impl<'src> Gluer<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self { lexer }
    }
    pub fn snapshot(&self) -> Self {
        Gluer {
            lexer: self.lexer.snapshot(),
        }
    }
    fn peek(&mut self) -> (Trivia, TokenKind, SmolStr) {
        self.peek_nth(0)
    }
    fn peek_nth(&mut self, n: usize) -> (Trivia, TokenKind, SmolStr) {
        let mut l = self.lexer.snapshot();
        let mut ret = l.next();
        for _ in 0..n {
            ret = l.next();
        }
        ret
    }
    pub fn collect(&mut self) -> TokenStream {
        self.collect_until_after(TokenKind::Eof)
    }
    pub fn collect_until_after(&mut self, kind: TokenKind) -> TokenStream {
        let mut stream = TokenStream::default();
        if self.peek().1 == kind {
            let triv = self.lexer.next().0;
            stream.tokens.push_trivia(triv);
            return stream;
        }
        let L(t1, tt) = self.next();
        stream.t1 = t1;
        stream.tokens = List::single(tt);
        while self.peek().1 != kind {
            let L(t, tt) = self.next();
            stream.tokens.push(t, tt);
        }
        let triv = self.lexer.next().0;
        stream.tokens.push_trivia(triv);
        stream
    }
    pub fn next(&mut self) -> WithLeadingTrivia<TokenTree> {
        let (t0, tok, s) = self.lexer.next();
        let tt = match tok {
            TokenKind::OpenBrace | TokenKind::OpenParen | TokenKind::OpenBracket => {
                #[rustfmt::skip]
                let (until, delim): (_, fn(_) -> _) = match tok {
                    TokenKind::OpenBrace   => (TokenKind::CloseBrace  , |stream: TokenStream| Delimited::Braces  (Braces  (stream))),
                    TokenKind::OpenParen   => (TokenKind::CloseParen  , |stream: TokenStream| Delimited::Parens  (Parens  (stream))),
                    TokenKind::OpenBracket => (TokenKind::CloseBracket, |stream: TokenStream| Delimited::Brackets(Brackets(stream))),
                    _ => unreachable!(),
                };
                TokenTree::Group(Box::new(delim(self.collect_until_after(until))))
            }
            TokenKind::At => TokenTree::Punct(Punct::At),
            TokenKind::Ident => TokenTree::Ident(Ident(s)),
            TokenKind::RawIdent => TokenTree::RawIdent(Ident(s)),
            TokenKind::Lifetime {
                starts_with_number: _,
            } => TokenTree::Lifetime(Ident(s)),
            TokenKind::RawLifetime => TokenTree::RawLifetime(Ident(s)),
            TokenKind::Literal {
                kind,
                suffix_start,
            } => {
                use ra_ap_rustc_lexer::LiteralKind as K;
                let kind = match kind {
                    K::Int { .. } => LiteralKind::Int,
                    K::Float { .. } => LiteralKind::Float,
                    _ => LiteralKind::Other,
                };
                let suffix_start = suffix_start as usize;
                let symbol = SmolStr::new(&s[..suffix_start]);
                let suffix = SmolStr::new(&s[suffix_start..]);
                TokenTree::Literal(Literal { kind, symbol, suffix })
            }
            TokenKind::Pound => TokenTree::Punct(Punct::Pound),
            TokenKind::Bang if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => TokenTree::Punct(Punct::BangEq),
            TokenKind::Bang => TokenTree::Punct(Punct::Bang),
            TokenKind::Semi => TokenTree::Punct(Punct::Semi),
            TokenKind::Colon if matches!(self.peek(), (t, TokenKind::Colon, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::ColonColon)
            }
            TokenKind::Colon => TokenTree::Punct(Punct::Colon),
            TokenKind::Eq if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::EqEq)
            }
            TokenKind::Eq if matches!(self.peek(), (t, TokenKind::Gt, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::RFatArrow)
            }
            TokenKind::Eq => TokenTree::Punct(Punct::Eq),
            TokenKind::Tilde => TokenTree::Punct(Punct::Tilde),
            TokenKind::Dollar => TokenTree::Punct(Punct::Dollar),
            TokenKind::Percent if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::PercentEq)
            }
            TokenKind::Percent => TokenTree::Punct(Punct::Percent),
            TokenKind::Caret if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::CaretEq)
            }
            TokenKind::Caret => TokenTree::Punct(Punct::Caret),
            TokenKind::And if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::AndEq)
            }
            TokenKind::And => TokenTree::Punct(Punct::And),
            TokenKind::Or if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::OrEq)
            }
            TokenKind::Or if matches!(self.peek(), (t, TokenKind::Or, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::OrOr)
            }
            TokenKind::Or => TokenTree::Punct(Punct::Or),
            TokenKind::Star if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::StarEq)
            }
            TokenKind::Star => TokenTree::Punct(Punct::Star),
            TokenKind::Eof => TokenTree::Eof,
            TokenKind::Plus if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::PlusEq)
            }
            TokenKind::Plus => TokenTree::Punct(Punct::Plus),
            TokenKind::Minus if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::MinusEq)
            }
            TokenKind::Minus if matches!(self.peek(), (t, TokenKind::Gt, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::RThinArrow)
            }
            TokenKind::Minus => TokenTree::Punct(Punct::Minus),
            TokenKind::Slash if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::SlashEq)
            }
            TokenKind::Slash => TokenTree::Punct(Punct::Slash),
            TokenKind::CloseBrace | TokenKind::CloseBracket | TokenKind::CloseParen => {
                panic!("unclosed group");
            }
            TokenKind::LineComment { .. }
            | TokenKind::BlockComment { .. }
            | TokenKind::Whitespace => {
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
            TokenKind::Dot if matches!(self.peek(), (t, TokenKind::Dot, _) if t.is_empty()) && matches!(self.peek_nth(1), (t, TokenKind::Dot, _) if t.is_empty()) => {
                self.lexer.next();
                self.lexer.next();
                TokenTree::Punct(Punct::DotDotDot)
            }
            TokenKind::Dot if matches!(self.peek(), (t, TokenKind::Dot, _) if t.is_empty()) && matches!(self.peek_nth(1), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                self.lexer.next();
                TokenTree::Punct(Punct::DotDotEq)
            }
            TokenKind::Dot if matches!(self.peek(), (t, TokenKind::Dot, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::DotDot)
            }
            TokenKind::Dot => TokenTree::Punct(Punct::Dot),
            TokenKind::Gt if matches!(self.peek(), (t, TokenKind::Gt, _) if t.is_empty()) && matches!(self.peek_nth(1), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                self.lexer.next();
                TokenTree::Punct(Punct::GtGtEq)
            }
            TokenKind::Lt if matches!(self.peek(), (t, TokenKind::Lt, _) if t.is_empty()) && matches!(self.peek_nth(1), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                self.lexer.next();
                TokenTree::Punct(Punct::LtLtEq)
            }
            TokenKind::Lt if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::LtEq)
            }
            TokenKind::Lt if matches!(self.peek(), (t, TokenKind::Minus, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::LThinArrow)
            }
            TokenKind::Gt if matches!(self.peek(), (t, TokenKind::Eq, _) if t.is_empty()) => {
                self.lexer.next();
                TokenTree::Punct(Punct::GtEq)
            }
            TokenKind::Lt => TokenTree::Punct(Punct::Lt),
            TokenKind::Gt => TokenTree::Punct(Punct::Gt),
            TokenKind::Comma => TokenTree::Punct(Punct::Comma),
            TokenKind::Question => TokenTree::Punct(Punct::Question),
        };

        t0 << tt
    }
}
