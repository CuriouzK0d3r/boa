extern crate boa;
use boa::exec::{Executor, Interpreter};
use boa::syntax::lexer::Lexer;
use boa::syntax::parser::Parser;
use std::fs::read_to_string;
use boa::repl::REPL;
use std::env;
#[macro_use]
extern crate clap;
use clap::App;

pub fn main() {
    let yaml = load_yaml!("../../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let option = matches.value_of("option").unwrap();
    println!("{}", option);
    let args: Vec<_> = env::args().collect();
    // if args.len() == 2 {
    //     match args[1].to_owned() {
    //         "repl" => {
    //             let mut repl: REPL = REPL::new();
    //             repl.run();
    //         },
    //         "file" => {
    //             let buffer = read_to_string(args[2]).unwrap();
    //             let mut lexer = Lexer::new(&buffer);
    //             lexer.lex().unwrap();
    //             let tokens = lexer.tokens;

    //             // Setup executor
    //             let expr = Parser::new(tokens).parse_all().unwrap();

    //             let mut engine: Interpreter = Executor::new();
    //             let result = engine.run(&expr);
    //             match result {
    //                 Ok(v) => print!("{}", v),
    //                 Err(v) => print!("Error: {}", v),
    //             }
    //         },

    //     }
    // }
    // else {

    // }
}
