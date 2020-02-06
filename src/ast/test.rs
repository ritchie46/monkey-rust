#[cfg(test)]
mod test {
    use crate::ast::ast::*;
    use crate::ast::err::ParserError;
    use crate::lexer::Lexer;
    use std::io::ErrorKind;

    fn parse_program(input: &str) -> Result<Program, ParserError> {
        let mut lex = Lexer::new(&input);
        let mut par = Parser::new(&mut lex);
        par.parse_program()
    }

    #[test]
    fn test_parser_errors() {
        let input = "let x;";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::AssignmentExpected(_) => assert!(true),
            _ => assert!(false)
        }
        let input = "let =";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::IdentifierExpected => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_() {
        let input = "let x = 5;
        let y = 10;
        let foobar = 838383;";
        let parsed = parse_program(&input);

        println!("{:?}", parsed)
    }
}
