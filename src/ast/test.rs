#[cfg(test)]
mod test {
    use crate::ast::ast::Parser;
    use crate::lexer::Lexer;

    #[test]
    fn test_() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";

        let mut lex = Lexer::new(&input);
        let mut par = Parser::new(&mut lex);
        let program = par.parse_program();

        println!("{:?}", program)
    }
}
