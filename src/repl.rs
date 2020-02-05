use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() {
    let io_in = io::stdin();
    loop {
        // Use stdout() instead of print! macro
        // Print macro gets flushed when new line is encountered
        io::stdout().write_all([PROMPT, " "].concat().as_bytes());
        Write::flush(&mut io::stdout());

        let mut s = String::new();
        io_in.read_line(&mut s);
        let mut lex = Lexer::new(&s);

        loop {
            let token = lex.next_token();
            match token.type_ {
                TokenType::EOF => return,
                _ => println!("{:?}", token),
            }
        }
    }
}
