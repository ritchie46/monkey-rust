use monkey::{
    eval::{
        environment::{Env, Environment},
        evaluator::eval_program,
        object::Object,
    },
    repl, Lexer, Parser,
};
use std::env;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(
            "Hello {}! This is the Monkey programming language!",
            whoami::username()
        );
        repl::start()
    } else {
        let t0 = SystemTime::now();
        let opt = std::fs::read_to_string(&args[1]);

        let s = match opt {
            Ok(s) => s,
            _ => {
                println!("could not read file");
                return;
            }
        };

        let mut env = Environment::new();
        let mut lex = Lexer::new(&s);
        let mut par = Parser::new(&mut lex);
        let parse_result = par.parse_program();

        match parse_result {
            Ok(program_ast) => println!("{}", eval_program(&program_ast, &mut env)),
            Err(e) => println!("{}", e),
        }
        let t1 = SystemTime::now();
        println!("Monkey program ran: {:?}", t1.duration_since(t0).unwrap())
    }
}
