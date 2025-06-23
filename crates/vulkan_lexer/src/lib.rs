pub mod stream;
mod tests;
pub mod token_types;

use crate::stream::CharStream;

use crate::token_types::*;

pub struct Lexer<'input>(CharStream<'input>);

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self(CharStream::new(input))
    }

    pub fn advance(&mut self) -> Token {
        self.match_token()
    }

    fn match_token(&mut self) -> Token {
        while let Some(r#char) = self.0.peek() {
            match r#char {
                _ if Self::is_whitespace(r#char) => {
                    self.0.next();
                    continue;
                }
                _ if r#char.is_alphabetic() => return self.match_keyword(),
                '0'..='9' => return self.match_literal(),
                SEMI => return self.match_punctuator(TokenKind::Semi),
                COMMA => return self.match_punctuator(TokenKind::Comma),
                DOT => return self.match_punctuator(TokenKind::Dot),
                LPAREN => return self.match_punctuator(TokenKind::LParen),
                RPAREN => return self.match_punctuator(TokenKind::RParen),
                LBRACE => return self.match_punctuator(TokenKind::LBrace),
                RBRACE => return self.match_punctuator(TokenKind::RBrace),
                LBRACKET => return self.match_punctuator(TokenKind::LBracket),
                RBRACKET => return self.match_punctuator(TokenKind::RBracket),
                AT => return self.match_punctuator(TokenKind::At),
                HASH => return self.match_punctuator(TokenKind::Hash),
                COLON => return self.match_punctuator(TokenKind::Colon),
                QMARK => return self.match_punctuator(TokenKind::QMark),
                EXCL => return self.match_punctuator(TokenKind::Excl),
                AMP => return self.match_punctuator(TokenKind::Amp),
                PIPE => return self.match_punctuator(TokenKind::Pipe),
                CARET => return self.match_punctuator(TokenKind::Caret),
                TILDE => return self.match_punctuator(TokenKind::Tilde),
                PLUS => return self.match_punctuator(TokenKind::Plus),
                MINUS => return self.match_punctuator(TokenKind::Minus),
                STAR => return self.match_punctuator(TokenKind::Star),
                SLASH => return self.match_punctuator(TokenKind::Slash),
                PERCENT => return self.match_punctuator(TokenKind::Percent),
                EQ => return self.match_punctuator(TokenKind::Eq),
                LT => return self.match_punctuator(TokenKind::Lt),
                GT => return self.match_punctuator(TokenKind::Gt),
                QUOTE => return self.match_punctuator(TokenKind::Quote),
                UNDERSCORE => return self.match_punctuator(TokenKind::Underscore),
                DOLLAR => return self.match_punctuator(TokenKind::Dollar),
                _ => return self.match_punctuator(TokenKind::Unknown),
            }
        }
        Token::new(TokenKind::EOF, self.0.offset(), self.0.offset())
    }
    #[inline]
    fn match_keyword(&mut self) -> Token {
        let start = self.0.offset();
        while let Some(c) = self.0.peek() {
            if !c.is_alphabetic() {
                break;
            }
            self.0.next();
        }
        Token::new(TokenKind::Ident, start, self.0.offset())
    }

    #[inline]
    fn match_literal(&mut self) -> Token {
        let start = self.0.offset();
        self.0.next();
        let mut base = Base::Decimal;
        let mut r#type: LiteralKind = LiteralKind::Int(Base::Decimal);

        if self.0.prev() == '0' && {
            match self.0.peek() {
                Some('b') => {
                    base = Base::Binary;
                    r#type = LiteralKind::Int(base.clone());
                    true
                }
                _ => false,
            }
        } {
            self.0.next();
        }

        while let Some(c) = self.0.peek() {
            let flag = match base {
                Base::Binary => c.is_digit(2),
                Base::Decimal => c.is_ascii_digit(),
            };

            if !flag {
                break;
            }
            self.0.next();
        }

        Token::new(TokenKind::Literal(r#type), start, self.0.offset())
    }

    #[inline]
    fn match_punctuator(&mut self, kind: TokenKind) -> Token {
        let start = self.0.offset();
        self.0.next();
        Token::new(kind, start, self.0.offset())
    }

    #[inline]
    pub fn is_whitespace(r#char: char) -> bool {
        matches!(
            r#char,
            '\u{0009}'
                | '\u{000A}'
                | '\u{000B}'
                | '\u{000C}'
                | '\u{000D}'
                | '\u{0020}'
                | '\u{0085}'
                | '\u{00A0}'
                | '\u{1680}'
                | '\u{2000}'
                ..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
        )
    }
}
