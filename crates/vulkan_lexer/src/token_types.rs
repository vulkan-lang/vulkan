#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident, // Identificator
    Literal(LiteralKind),
    Semi,       // Semicolon, ;
    Comma,      // Comma, ,
    Dot,        // Period, .
    LParen,     // LeftParenthesis, (
    RParen,     // RightParenthesis, )
    LBrace,     // LeftBrace, {
    RBrace,     // RightBrace, }
    LBracket,   // LeftBracket, [
    RBracket,   // RightBracket, ]
    At,         // AtSign, @
    Hash,       // Hash, #
    Colon,      // Colon, :
    QMark,      // QuestionMark, ?
    Excl,       // ExclamationMark, !
    Amp,        // Ampersand, &
    Pipe,       // VerticalBar, |
    Caret,      // Caret, ^
    Tilde,      // Tilde, ~
    Plus,       // Plus, +
    Minus,      // Minus, -
    Star,       // Asterisk, *
    Slash,      // Slash, /
    Percent,    // Percent, %
    Eq,         // Equals, =
    Lt,         // LessThan, <
    Gt,         // GreaterThan, >
    Backslash,  // Backslash, \
    Quote,      // Quote, '
    Underscore, // Underscore, _
    Dollar,     // DollarSign, $
    Unknown,    // Unknown TokenKind
    EOF,        // End Of File,
}

pub const SEMI: char = ';';
pub const COMMA: char = ',';
pub const DOT: char = '.';
pub const LPAREN: char = '(';
pub const RPAREN: char = ')';
pub const LBRACE: char = '{';
pub const RBRACE: char = '}';
pub const LBRACKET: char = '[';
pub const RBRACKET: char = ']';
pub const AT: char = '@';
pub const HASH: char = '#';
pub const COLON: char = ':';
pub const QMARK: char = '?';
pub const EXCL: char = '!';
pub const AMP: char = '&';
pub const PIPE: char = '|';
pub const CARET: char = '^';
pub const TILDE: char = '~';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const STAR: char = '*';
pub const SLASH: char = '/';
pub const PERCENT: char = '%';
pub const EQ: char = '=';
pub const LT: char = '<';
pub const GT: char = '>';
pub const QUOTE: char = '\'';
pub const DQUOTE: char = '"';
pub const UNDERSCORE: char = '_';
pub const DOLLAR: char = '$';

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int(Base),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base {
    Binary,
    Decimal,
}
