use crate::js::value::{from_value, to_value, ResultValue, Value, ValueData};
use crate::syntax::ast::expr::{Expr, ExprDef};
use crate::exec::{Interpreter, Executor};
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
extern crate rustyline;

pub struct REPL {
    pub intr: Interpreter,
    pub editor: rustyline::Editor::<()>,
}

impl REPL {
    pub fn new() -> REPL {
        let mut rl = rustyline::Editor::<()>::new();
        let mut engine: Interpreter = Executor::new();
        REPL {
            intr : engine,
            editor : rl,
        }
    }

    pub fn run(&mut self) -> (){
        loop {
            // let mut parser;
            match self.editor.readline("> ") {
                Ok(line) => {
                    let mut lexer = Lexer::new(&line);
                    lexer.lex().unwrap();
                    let tokens = lexer.tokens;

                    let expr = Parser::new(tokens).parse_all().unwrap().expect("fa");
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