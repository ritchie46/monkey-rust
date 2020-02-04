use crate::token::token::{Token, TokenType};

struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    ch: u8
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
       let mut lex = Lexer{input: input.as_bytes(),
           position: 0,
           ch: 0};

        lex.read_next_char();
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
                   Token{
                       type_: EOF,
                       literal: String::from("")
                   }
               } else {
                   new_token(Illegal, self.ch)
               }

            }
        };
        self.read_next_char();
        token
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        type_: token_type,
        literal: std::str::from_utf8(&[ch]).unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "=+(){},;";

        use TokenType::*;
        let valid = [
            (Plus, "+"),
            (LParen, "("),
            (RParen, ")"),
            (LBrace, "{"),
            (RBrace, "}"),
            (Comma, ","),
            (Semicolon, ";"),
            (EOF, "")
        ];

        let mut lex = Lexer::new(input);
        for (type_, literal) in valid.iter() {
            let t = Token {
                type_: *type_,
                literal: literal.to_string()
            };
            assert_eq!(t, lex.next_token())
        }
    }

}
