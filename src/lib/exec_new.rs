use crate::js::function::{Function, RegularFunction};
use crate::js::object::{INSTANCE_PROTOTYPE, PROTOTYPE};
use crate::js::value::{from_value, to_value, ResultValue, Value, ValueData};
use crate::js::{array, console, function, json, math, object, string};
use crate::syntax::ast::constant::Const;
use crate::syntax::ast::expr::{Expr, ExprDef};
use crate::syntax::ast::op::{BinOp, BitOp, CompOp, LogOp, NumOp, UnaryOp};
use gc::{Gc, GcCell};
use std::borrow::Borrow;
use std::collections::HashMap;
use ratel::ast::Pattern;
use ratel::ast::operator::*;

extern crate ratel;
use ratel::ast::expression::*;
pub use ratel::ast::literal::Literal;
use ratel::ast::Statement;
use ratel::parse;

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

    fn runExpr(&mut self, expr: &Expression) -> Value;
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

    fn runExpr(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Literal(i) => match i {
                Literal::Number(num) => to_value(num.parse::<f64>().ok()),
                Literal::Binary(num) => to_value(num.parse::<f64>().ok()),
                Literal::String(str_val) => to_value(str_val.to_owned()),
                Literal::RegEx(regex) => to_value(None::<()>),
                Literal::False => to_value(false),
                Literal::True => to_value(true),
                Literal::Null => to_value(None::<()>),
                Literal::Undefined => Gc::new(ValueData::Undefined),
            },
            Expression::Identifier(id) => to_value(id.to_owned()),
            Expression::Binary(e) => {
                match e.operator {
                    OperatorKind::Addition => {
                        let left  =  self.runExpr(&e.left.item);
                        let right =  self.runExpr(&e.right.item);
                        let mut val;
                        if (left.is_double() && right.is_double()) {
                            val = left.to_num() + right.to_num();
                        } 
                        else {
                            val = -1.0;
                        }
                        to_value(val)
                    },
                    _ => panic!("unhandled case"),    
                }
            },
            _ => panic!("unhandled case"),
        }
    }

    fn run(&mut self, stmt: &Statement) -> () {
        match stmt {
            Statement::Expression(e) => println!("{}", self.runExpr(&e.item)),
            Statement::Declaration(v) => {
                let decl = v.declarators;
                let kind = v.kind;
                 let scope_vars = self.scope().vars.clone();
                let scope_vars_ptr = scope_vars.borrow();
                 for i in &decl {
                    println!("trace3 : {:#?}", i);
                    let id = i.item.id.item;
                    let init = i.item.init;
                    let val = match init {
                        Some(l) => self.runExpr(&l.item),
                        _ => panic!("unhandled case"),
                    };

                    let varname = match id {
                        Pattern::Identifier(name) => name,
                        _ => panic!("wrong var name"),
                    };
 
                    scope_vars_ptr.set_field(varname.to_owned(), val);
                 }
            },
            Statement::Block(b) => {
                let body = b.body;
                 for i in &body {
                     self.run(&i.item);
                 }
                //  body.last().unwrap().clone();
            },
            _ => println!("fd"),
        };
        ()
    }
}
