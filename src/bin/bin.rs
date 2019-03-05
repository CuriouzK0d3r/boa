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

fn printHelp() {
    println!("USAGE:");
    println!("\t ./bin <mode> [INPUT]");
}

pub fn main() {

    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => printHelp(),
        2 => match args[1].as_ref() {
            "repl" => {
                let mut repl: REPL = REPL::new();
                repl.run();
            },
            "help" => {
                printHelp();
            },
            _ => printHelp(),
        },
        3 => match args[1].as_ref() {
            "run" => {
                let buffer = read_to_string(args[2]).unwrap();
                let mut lexer = Lexer::new(&buffer);
                lexer.lex().unwrap();
                let tokens = lexer.tokens;

                // Setup executor
                let expr = Parser::new(tokens).parse_all().unwrap();

                let mut engine: Interpreter = Executor::new();
                let result = engine.run(&expr);
                match result {
                    Ok(v) => print!("{}", v),
                    Err(v) => print!("Error: {}", v),
                }
            },
            _ => printHelp(),
        },
    }
}