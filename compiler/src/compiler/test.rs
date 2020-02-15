use monkey::{Lexer, ParseResult, Parser, ParserError, Program};

fn parse(input: &str) -> Result<Program, ParserError> {
    let mut lex = Lexer::new(input);
    let mut par = Parser::new(&mut lex);
    return par.parse_program();
}

#[test]
fn test_compiler() {}
