#[cfg(test)]
mod tests {
    use crate::{Lexer, stream::CharStream, token_types::TokenKind};

    const PRINT_FLAG: bool = true;

    #[test]
    fn test_char_stream() {
        let input = "Hello, World!";
        let mut stream = CharStream::new(input);

        let parsed: Vec<char> = std::iter::from_fn(|| stream.next()).collect();

        if PRINT_FLAG {
            println!("[test_char_stream]: {:?}", parsed);
        }

        let input_chars: Vec<char> = input.chars().collect();

        assert_eq!(parsed, input_chars);
    }

    #[test]
    fn test_lexer_token_stream() {
        let input = r#"
use std:io/macros::println;

fn main() {
    println(10 + 10);
}
"#;

        let mut lexer = Lexer::new(input);

        std::iter::from_fn(|| {
            let token = lexer.advance();
            (!matches!(token.kind, TokenKind::EOF)).then_some(token)
        })
        .for_each(|token| {
            let lexeme = input.get(token.start..token.end).expect("fail");

            if PRINT_FLAG {
                println!(
                    "[test_lexer_token_stream]: Token: '{}', Kind: '{:?}'",
                    lexeme, token.kind
                );
            }
        });
    }
}
