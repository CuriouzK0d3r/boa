use crate::exec_new::{Executor, Interpreter};
extern crate ratel;
extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;
extern crate toolshed;
use ratel::parse;

/// Starting prompt
const DEFAULT_PROMPT: &'static str = "js => ";
const MORE_PROMPT: &'static str = "...   ";

pub struct REPL {
    pub intr: Interpreter,
    pub editor: rustyline::Editor<()>,
}

impl REPL {
    pub fn new() -> REPL {
        let rl = rustyline::Editor::<()>::new();
        let engine: Interpreter = Executor::new();
        REPL {
            intr: engine,
            editor: rl,
        }
    }

    pub fn run(&mut self) -> () {

        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        let mut prompt = DEFAULT_PROMPT;
        let mut last_command : String = "".to_string();

        loop {
            match rl.readline(prompt) {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());
                    
                    last_command.push_str(&line);

                    let copy = last_command.clone();
                    let r = parse(&copy);

                    match r {
                        Ok(module) => {
                            let vec = module.body();

                            for i in &vec {
                                let x = i.item;
                                let mut engine: Interpreter = Executor::new();
                                engine.run(&x);
                            }
                            rl.save_history("history.txt").unwrap();
                            prompt = DEFAULT_PROMPT;
                            last_command = "".to_string();
                        },
                        Err(_) => {
                            prompt = MORE_PROMPT;
                            last_command.push_str(&line);
                        },
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    break
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    }
}