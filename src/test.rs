#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::err::ParserError;
    use crate::lexer::Lexer;
    use crate::parser::*;

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
            _ => assert!(false),
        }
        let input = "let =";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::IdentifierExpected => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_precedence() {
        assert!(Precedence::Lowest < Precedence::Equals);
        assert!(Precedence::Prefix == Precedence::Prefix)
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::Identifier("foobar".to_string())),
            parsed.unwrap().statements[0]
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::IntegerLiteral(5 as i64)),
            parsed.unwrap().statements[0]
        );
    }
}
