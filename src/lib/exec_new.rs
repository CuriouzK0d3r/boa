use crate::js::value::{to_value, Value, ValueData};
use crate::js::{array, console, function, json, math, object, string};
use gc::Gc;
use std::borrow::Borrow;
use ratel::ast::Pattern;
use ratel::ast::operator::*;
use ratel::ast::function::MandatoryName;

extern crate ratel;
use ratel::ast::expression::*;
pub use ratel::ast::literal::Literal;
use ratel::ast::Statement;

/// A variable scope
#[derive(Trace, Finalize, Clone, Debug)]
pub struct Scope {
    /// The value of `this` in the scope
    pub this: Value,
    /// The variables declared in the scope
    pub vars: Value,
}

/// An execution engine
pub trait Executor {
    /// Make a new execution engine
    fn new() -> Self;
    /// Set a global variable called `name` with the value `val`
    fn set_global(&mut self, name: String, val: Value) -> Value;
    /// Resolve the global variable `name`
    fn get_global(&self, name: String) -> Value;
    /// Create a new scope and return it
    fn make_scope(&mut self, this: Value) -> Scope;
    /// Destroy the current scope
    fn destroy_scope(&mut self) -> Scope;
    /// Run an expression
    fn run(&mut self, expr: &Statement) -> ();

    fn run_expr(&mut self, expr: &Expression) -> Value;
}

/// A Javascript intepreter
pub struct Interpreter {
    /// An object representing the global object
    global: Value,
    /// The scopes
    pub scopes: Vec<Scope>,
    pub decls: std::collections::HashMap<String, Value> 

}

impl Interpreter {
    #[inline(always)]
    /// Get the current scope
    pub fn scope(&self) -> &Scope {
        self.scopes.get(self.scopes.len() - 1).unwrap()
    }
}

impl Executor for Interpreter {
    fn new() -> Interpreter {
        let global = ValueData::new_obj(None);
        object::init(global.clone());
        console::init(global.clone());
        math::init(global.clone());
        array::init(global.clone());
        function::init(global.clone());
        json::init(global.clone());
        string::init(global.clone());
        Interpreter {
            global: global.clone(),
            decls: std::collections::HashMap::new().clone(),
            scopes: vec![Scope {
                this: global.clone(),
                vars: global.clone()
            }],
        }
    }

    fn set_global(&mut self, name: String, val: Value) -> Value {
        self.global.borrow().set_field(name, val)
    }

    fn get_global(&self, name: String) -> Value {
        self.global.borrow().get_field(name)
    }

    fn make_scope(&mut self, this: Value) -> Scope {
        let scope = Scope {
            this: this,
            vars: ValueData::new_obj(None)
        };
        self.scopes.push(scope.clone());
        scope
    }

    fn destroy_scope(&mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    fn run_expr(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Literal(i) => match i {
                Literal::Number(num) => to_value(num.parse::<f64>().ok()),
                Literal::Binary(num) => to_value(num.parse::<f64>().ok()),
                Literal::String(str_val) => to_value(format!("{}", (str_val.replace("\"", "")))),
                Literal::RegEx(_regex) => to_value(None::<()>),
                Literal::False => to_value(false),
                Literal::True => to_value(true),
                Literal::Null => to_value(None::<()>),
                Literal::Undefined => Gc::new(ValueData::Undefined),
            },
            Expression::Call(_e) => {
                // let args = e.arguments;
                // let name = self.run_expr(&e.callee.item);
                to_value(3)
            },
            Expression::Identifier(id) => {let scope_vars = self.scope().vars.borrow(); scope_vars.get_field(id.to_string())},
            Expression::Function(_e) => {to_value(3)},
            Expression::Binary(e) => {
                match e.operator {
                    OperatorKind::Addition => {
                        let left  =  self.run_expr(&e.left.item);
                        let right =  self.run_expr(&e.right.item);

                        if left.is_double() && right.is_double() {
                            to_value(left.to_num() + right.to_num())
                        } 
                        else if left.is_string() && right.is_double() {
                            let owned_string = format!("{}{}", left, right.to_num());
                            to_value(format!("{}", owned_string))
                        }
                        else {
                            Gc::new(ValueData::Undefined)
                        }
                    },
                    OperatorKind::Subtraction => {
                        let left  =  self.run_expr(&e.left.item);
                        let right =  self.run_expr(&e.right.item);

                        if left.is_double() && right.is_double() {
                            to_value(left.to_num() - right.to_num())
                        } 
                        else {
                            Gc::new(ValueData::Undefined)
                        }
                    },
                    OperatorKind::Division => {
                        let left  =  self.run_expr(&e.left.item);
                        let right =  self.run_expr(&e.right.item);

                        if left.is_double() && right.is_double() {
                            if right.to_num() == 0.0 {
                                panic!("Error! Division By Zero!");
                            }
                            to_value(left.to_num() / right.to_num())
                        } 
                        else {
                            Gc::new(ValueData::Undefined)
                        }
                    },
                    OperatorKind::Multiplication => {
                        let left  =  self.run_expr(&e.left.item);
                        let right =  self.run_expr(&e.right.item);

                        if left.is_double() && right.is_double() {
                            to_value(left.to_num() * right.to_num())
                        } 
                        else {
                            Gc::new(ValueData::Undefined)
                        }
                    }
                    _ => panic!("unhandled case"),    
                }
            },
            _ => panic!("unhandled case"),
        }
    }

    fn run(&mut self, stmt: &Statement) -> () {
        match stmt {
            Statement::Expression(e) => println!("{}", self.run_expr(&e.item)),
            Statement::Declaration(v) => {
                let decl = v.declarators;
                let _kind = v.kind;
                let scope_vars = self.scope().vars.clone();
                let scope_vars_ptr = scope_vars.borrow();
                 for i in &decl {
                    let id = i.item.id.item;
                    let init = i.item.init;
                    let val = match init {
                         Some(l) => self.run_expr(&l.item),
                         _ => panic!("unhandled case"),
                    };

                    let varname = match id { 
                        Pattern::Identifier(name) => name,
                         _ => panic!("wrong var name"),
                     };
                     
                     scope_vars_ptr.set_field(varname.to_string(), val.clone());
                     self.set_global(varname.to_string(), val.clone());
                     &self.decls.insert(varname.to_string(), val.clone());
                 }
            },
            Statement::Block(b) => {
                let body = b.body;
                 for i in &body {
                     self.run(&i.item);
                 }
                //  body.last().unwrap().clone();
            },
            Statement::If(exp) => {
                let _cond = self.run_expr(&exp.test.item);
                let _cons = exp.consequent;
                let _alternate = exp.alternate;

                if _cond.is_true() {
                    self.run(&_cons.item);
                }
                else {
                    match _alternate {
                        Some(e) => self.run(&e.item),
                        _ => (),
                    }
                }
            },
            Statement::While(exp) => {
                let _cond = self.run_expr(&exp.test.item);
                let _cons = exp.body;
                while _cond.is_true() {
                    self.run(&_cons.item);
                }
                println!("{:?}",exp);
            },
            Statement::Function(e) => {
                let _params    = e.params;
                let _body      = e.body.item.body;

                let _func_name = match e.name {
                    MandatoryName(name) => name.item,
                };

                // self.run(&_body.item.body);
                let mut v = Vec::new();
                for i in &_body {
                    self.run(&i);
                    v.push(i);
                }

                let mut memory = std::collections::HashMap::new();
                // scope_vars.set_field(_func_name.to_string(), v);

                memory.insert(
                    _func_name.to_string(),
                    _body,
                );



                
                // let val = crate::js::function::NewRegularFunction::new(Statement::Function(e), new Vec<string)();
                // self.global
                //     .borrow()
                //     .set_field(_func_name.to_owned().clone(), val.clone());
            },
            Statement::Empty => (),
            _ => panic!("unhandled case"),
        };
        ()
    }
}
