use gc::{Gc, GcCell};
use crate::js::function::{Function, RegularFunction};
use crate::js::object::{INSTANCE_PROTOTYPE, PROTOTYPE};
use crate::js::value::{from_value, to_value, ResultValue, Value, ValueData};
use crate::js::{array, console, function, json, math, object, string};
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::syntax::ast::constant::Const;
use crate::syntax::ast::expr::{Expr, ExprDef};
use crate::syntax::ast::op::{BinOp, BitOp, CompOp, LogOp, NumOp, UnaryOp};

extern crate ratel;
use ratel::parse;
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
}

/// A Javascript intepreter
pub struct Interpreter {
    /// An object representing the global object
    global: Value,
    /// The scopes
    pub scopes: Vec<Scope>,
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
            scopes: vec![Scope {
                this: global.clone(),
                vars: global.clone(),
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
            vars: ValueData::new_obj(None),
        };
        self.scopes.push(scope.clone());
        scope
    }

    fn destroy_scope(&mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    fn run(&mut self, stmt : &Statement) -> () {

        match stmt {                    
            Statement::Expression(e) => match e.item {
                Expression::Literal(i) => match i {
                       Literal::Number(num) => println!("{}", to_value(num)),
                       Literal::Binary(num) => println!("{}", to_value(num)),
                       Literal::String(str_val) => println!("{}", to_value(str_val)),
                       Literal::RegEx(regex) => println!("{}", to_value(regex)),
                       Literal::False => println!("{}", to_value(false)),
                       Literal::True => println!("{}", true),
                       Literal::Null => println!("{}", to_value(None::<()>)),
                       Literal::Undefined => println!("{}", Gc::new(ValueData::Undefined)),
                    },
                _ => println!("df"),
            },
            Statement::Block(b) => println!("{:#?}", b),
            _ => println!("fd"),
        };
        ()
    }
}
