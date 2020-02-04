use crate::token::token::{Token, TokenType, KEYWORDS};

struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        let mut lex = Lexer {
            input: input.as_bytes(),
            position: 0,
            ch: 0,
        };

        lex.ch = lex.input[lex.position];
        lex
    }

    fn read_position(&self) -> usize {
        self.position + 1
    }

    fn read_next_char(&mut self) {
        // Reads the next character w.r.t. current position.
        if self.read_position() >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position()]
        }
        self.position = self.read_position()
    }

    fn next_token(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();
        let token = match self.ch as char {
            '=' => new_token(Assign, self.ch),
            ';' => new_token(Semicolon, self.ch),
            '(' => new_token(LParen, self.ch),
            ')' => new_token(RParen, self.ch),
            ',' => new_token(Comma, self.ch),
            '+' => new_token(Plus, self.ch),
            '{' => new_token(LBrace, self.ch),
            '}' => new_token(RBrace, self.ch),
            _ => {
                if self.ch == 0 {
                    Token {
                        type_: EOF,
                        literal: String::from(""),
                    }
                } else if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    // Early return because read_identifier has read to
                    // end of the identifier and we don't want to call
                    // read_next_char again.
                    return match KEYWORDS.get(&identifier) {
                        Some(keyword) => Token {
                            type_: *keyword,
                            literal: identifier,
                        },
                        _ => Token {
                            type_: Identifier,
                            literal: identifier,
                        },
                    };
                } else {
                    new_token(Illegal, self.ch)
                }
            }
        };
        self.read_next_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_next_char()
        }
        std::str::from_utf8(&self.input[position..self.position])
            .unwrap()
            .to_string()
    }
    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_next_char()
        }
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        type_: token_type,
        literal: std::str::from_utf8(&[ch]).unwrap().to_string(),
    }
}

fn is_letter(ch: u8) -> bool {
    match (ch as char).to_lowercase().next() {
        Some(ch) => ('a' <= ch) && (ch <= 'z'),
        None => false,
    }
}

fn is_whitespace(ch: u8) -> bool {
    match (ch as char).to_lowercase().next() {
        Some(ch) => (ch == ' ') || (ch == '\t') || (ch == '\r') || (ch == '\n'),
        None => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "=+(){} ,;
        let five = 5";

        use TokenType::*;
        let valid = [
            (Assign, "="),
            (Plus, "+"),
            (LParen, "("),
            (RParen, ")"),
            (LBrace, "{"),
            (RBrace, "}"),
            (Comma, ","),
            (Semicolon, ";"),
            (Let, "let"),
            (Identifier, "five"),
            (Assign, "="),
            (EOF, ""),
        ];

        let mut lex = Lexer::new(input);
        for (type_, literal) in valid.iter() {
            let t = Token {
                type_: *type_,
                literal: literal.to_string(),
            };
            assert_eq!(t, lex.next_token())
        }
    }
}
