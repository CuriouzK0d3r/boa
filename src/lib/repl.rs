use crate::exec_new::{Interpreter, Executor};
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
extern crate ratel;
extern crate rustyline;
use std::panic;
extern crate esprit;
extern crate toolshed;
use crate::js::value::{from_value, to_value, ResultValue, Value, ValueData};

// use toolshed::list::List;
// use ratel::ast;
use ratel::parse;
use ratel::ast::expression::*;
pub use ratel::ast::literal::Literal;
use ratel::ast::Statement;

// fn tokenize(b: &mut Bencher) {
//     let arena = toolshed::Arena::new();
//     let ptr = arena.alloc_str_with_nul(SOURCE);
//     b.bytes = SOURCE.len() as u64;

//     b.iter(|| {
//         let mut lexer = unsafe { ratel::lexer::Lexer::from_ptr(ptr) };

//         while lexer.token != ratel::lexer::Token::EndOfProgram {
//             lexer.consume()
//         }
//     });
// }

/// Starting prompt
const DEFAULT_PROMPT: &'static str = "js => ";
/// Prompt when further input is being read
const MORE_PROMPT: &'static str = "js_i .> ";
/// Prompt when a `.block` command is in effect
const BLOCK_PROMPT: &'static str = "js_i +> ";

pub struct REPL {
    pub intr: Interpreter,
    pub editor: rustyline::Editor::<()>,
}

impl REPL {
    pub fn new() -> REPL {
        let rl = rustyline::Editor::<()>::new();
        let engine: Interpreter = Executor::new();
        REPL {
            intr : engine,
            editor : rl,
        }
    }

    pub fn run(&mut self) -> (){
        loop {
            let mut last_command: String = "".to_owned();
            // let mut history = [];

            match self.editor.readline(DEFAULT_PROMPT) {
                Ok(line) => {

                    // let result = panic::catch_unwind(|| {
                        let s = String::new(); // String type implements Clone
                        let copy = line.clone();
                        let r = parse(&copy);
                        // // let m = r[0].statements();
                        // // ratel::grammar::Statement::Empty
                        match r {
                            Ok(module) => {
                                let vec = module.body();
                                
                                for i in &vec {
                                    // println!("A reference to {:#?}", i.item);
                                    
                                    let x = i.item;
                                    // println!("{}", unsafe { std::intrinsics::type_name::<i>() });
                                    let mut engine: Interpreter = Executor::new();
                                    engine.run(&x);
                                    
                                }
                                // print!("{:#?}", module.body)
                            },
                            Err(v) => print!("Error: "),
                        }

                        // let r = esprit::script(&copy);
                        // match r {
                        //     Ok(actual_ast) => {
                        //         let stms = actual_ast.body;
                                
                        //     },
                        //     Err(v) => print!("Error: {:#?}", v),
                        // }
                        // let _module = ratel::parser::parse(copy);//.expect("Must parse");
                        // println!("{}", _module);
                        
                        // let mut lexer = Lexer::new(&line);
                        // lexer.lex().expect("no lex");
                        
                        // let tokens = lexer.tokens;

                        // let expr = Parser::new(tokens).parse_all().expect("no parse");
                        // let mut engine: Interpreter = Executor::new();
                        // let result = engine.run(&expr);
                        // match result {
                        //     Ok(v) => print!("{}", v),
                        //     Err(v) => print!("Error: {}", v),
                        // }
                    // });
                    // if result.is_err() {
                    //     last_command.push_str(&line);
                    //     println!("{}", last_command);
                    // }
                    
                    // match expr {
                    //     Ok(exp) => println!("ok"),
                    //     Err(error) => match error.kind() {
                    //         ErrorKind::AbruptEnd => println!("tinue"),
                    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
                    //     },
                    // }
                },
                Err(_) => break,
            }
        }
    }
}

// fn repl(trace: bool) {

//     let mut vm = vm::vm::VM::new();
//     vm.is_debug = trace;
//     let mut rl = rustyline::Editor::<()>::new();

//     loop {
//         let mut parser;
//         match rl.readline("> ") {
//             Ok(line) => {
//                 rl.add_history_entry(line.as_ref());
//                 let mut lines = line.clone() + "\n";
//                 loop {
//                     parser = parser::Parser::new(lines.clone());
//                     match parser.parse_all() {
//                         Ok(node) => {
//                             // compile and execute
//                             let mut iseq = vec![];
//                             match vm.codegen.compile(&node, &mut iseq, true) {
//                                 Ok(()) => {}
//                                 Err(vm_codegen::Error::General { msg, token_pos }) => {
//                                     parser.show_error_at(token_pos, msg.as_str());
//                                     break;
//                                 }
//                                 Err(vm_codegen::Error::Unimplemented { msg, token_pos }) => {
//                                     parser.show_error_at(token_pos, msg.as_str());
//                                     break;
//                                 }
//                             };

//                             let res = vm.run(iseq);
//                             if vm.state.stack.len() == 0 {
//                                 vm.state.stack.push(vm::value::Value::Undefined);
//                             };
//                             if vm.context_stack.len() != 0 {
//                                 println!(
//                                     "Warning: context length is {} (should be 0)",
//                                     vm.context_stack.len()
//                                 );
//                             };
//                             match res {
//                                 Err(e) => {
//                                     e.show_error_message();
//                                 }
//                                 _ => {
//                                     // Show the evaluated result
//                                     if let Some(value) = vm.state.stack.pop() {
//                                         if value == Value::Undefined {
//                                             print!(
//                                                 "{}",
//                                                 Colour::Fixed(8).paint(value.format(3, true))
//                                             );
//                                         } else {
//                                             print!("{}", value.format(3, true));
//                                         }

//                                         println!();
//                                         /*
//                                         unsafe {
//                                             builtin::debug_print(&value, true);
//                                             libc::puts(b"\0".as_ptr() as *const i8);
//                                         }
//                                         */
//                                     }
//                                 }
//                             };
//                             vm.state.stack.clear();
//                             break;
//                         }
//                         Err(parser::Error::UnexpectedEOF(_)) => match rl.readline("... ") {
//                             Ok(line) => {
//                                 rl.add_history_entry(line.as_ref());
//                                 lines += line.as_str();
//                                 lines += "\n";
//                                 continue;
//                             }
//                             Err(_) => break,
//                         },
//                         Err(_) => break,
//                     };
//                 }
//             }
//             Err(_) => break,
//         };
//     }
// }