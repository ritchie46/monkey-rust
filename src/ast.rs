use crate::token::Token;
use crate::lexer::Lexer;


trait Node {
    fn token_literal(&self) -> String;
}
trait Statement {
    fn statement_node();
}

trait Expression {
    fn expression_node();
}

struct Identifier {
    token: Token,
    value: String
}
impl Identifier {
    fn expresssion_ndoe() {}
}

struct LetStatement<E>
where E: Expression
{
    token: Token,
    name: Identifier,
    value: E
}

impl <E>Statement for LetStatement<E>
where E: Expression
{
    fn statement_node() {}
}

impl <T>Node for LetStatement<T>
where T: Node + Expression
{
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}


struct Program<T>
where T: Statement
{
    statements: Vec<T>
}


struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token
}
impl <'a>Parser<'a> {
    fn new(lex: &'a mut Lexer<'a>) -> Parser<'a> {

        let current = lex.next_token();
        let peek = lex.next_token();

        let p = Parser{
            lex,
            current_token: current,
            peek_token: peek
        };
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token().clone();
    }
}