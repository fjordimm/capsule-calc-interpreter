use std::{collections::HashMap, hash::Hash};

use dyn_clone::DynClone;

pub enum Term {
    Const(Constant),
    Var(Variable),
    Lambda(Variable, Option<Box<Term>>),
    Application(Option<Box<Term>>, Option<Box<Term>>),
    Assignment(Variable, Option<Box<Term>>),
}

pub fn t_const(value: Constant) -> Term {
    Term::Const(value)
}

pub fn t_var(var: Variable) -> Term {
    Term::Var(var)
}

pub fn t_lambda(param: Variable, body: Term) -> Term {
    Term::Lambda(param, Some(Box::new(body)))
}

pub fn t_application(t1: Term, t2: Term) -> Term {
    Term::Application(Some(Box::new(t1)), Some(Box::new(t2)))
}

pub fn t_assignment(var: Variable, value: Term) -> Term {
    Term::Assignment(var, Some(Box::new(value)))
}

/// Deep independent copy. Boxes clone their inner values, they do not copy over the pointer.
/// Exception: `ConstFunc`s clone over the box, they do not make a new one.
impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Self::Const(c) => Self::Const(c.clone()),
            Self::Var(v) => Self::Var(v.clone()),
            Self::Lambda(v, t) => Self::Lambda(v.clone(), t.to_owned()),
            Self::Application(t1, t2) => Self::Application(t1.to_owned(), t2.to_owned()),
            Self::Assignment(v, t) => Self::Assignment(v.clone(), t.to_owned()),
        }
    }
}

trait ConstFuncInner: Fn(&Constant) -> Term + DynClone {}

impl<F> ConstFuncInner for F where F: Fn(&Constant) -> Term + DynClone {}

pub enum Constant {
    Error,
    Unit,
    Bool(bool),
    Int(u32),
    Func {
        name: String,
        #[allow(private_interfaces)]
        f: Box<dyn ConstFuncInner>,
    },
}

impl Constant {
    pub fn as_str(&self) -> String {
        match self {
            Constant::Error => String::from("Error"),
            Constant::Unit => String::from("Unit"),
            Constant::Bool(value) => format!("Bool({})", value),
            Constant::Int(value) => format!("Int({})", value),
            Constant::Func { name, f: _ } => format!("Func(\"{}\")", name),
        }
    }
}

impl Clone for Constant {
    fn clone(&self) -> Self {
        match self {
            Self::Error => Self::Error,
            Self::Unit => Self::Unit,
            Self::Bool(arg0) => Self::Bool(arg0.clone()),
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Func { name, f } => Self::Func {
                name: name.clone(),
                f: dyn_clone::clone_box(&**f),
            },
        }
    }
}

pub fn c_error() -> Constant {
    Constant::Error
}

pub fn c_unit() -> Constant {
    Constant::Unit
}

pub fn c_bool(value: bool) -> Constant {
    Constant::Bool(value)
}

pub fn c_int(value: u32) -> Constant {
    Constant::Int(value)
}

#[allow(private_interfaces, private_bounds)]
pub fn c_func(name: impl Into<String>, f: impl ConstFuncInner + 'static) -> Constant {
    Constant::Func {
        name: name.into(),
        f: Box::new(f),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Variable(String);

impl Variable {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn variable(name: impl Into<String>) -> Variable {
    Variable(name.into())
}

// TODO: Is it fine that this is an unordered map? I think so...
pub struct Environment(HashMap<Variable, Box<Term>>);

impl Environment {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with(mut self, variable: Variable, value: Term) -> Self {
        self.add(variable, Box::new(value));
        self
    }

    pub fn add(&mut self, variable: Variable, value: Box<Term>) {
        self.0.insert(variable, value);
    }

    pub fn lookup(&self, variable: &Variable) -> Option<&Box<Term>> {
        self.0.get(variable)
    }

    pub fn lookup_and_take(&mut self, _variable: &Variable) -> Option<Box<Term>> {
        todo!()
    }

    pub fn fresh_var(&self) -> Variable {
        loop {
            let candidate = variable(format!("FRESH_{}", rand::random::<u64>()));
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
        }
    }
}

fn some_or_panic(t: Option<Box<Term>>) -> Box<Term> {
    match t {
        Some(t_s) => t_s,
        None => panic!("An Option<Box<Term>> was None! This should not happen."),
    }
}

fn some_or_panic_r(t: &Option<Box<Term>>) -> &Box<Term> {
    match t {
        Some(t_s) => t_s,
        None => panic!("An Option<Box<Term>> was None! This should not happen."),
    }
}

fn some_or_panic_rm(t: &mut Option<Box<Term>>) -> &mut Box<Term> {
    match t {
        Some(t_s) => t_s,
        None => panic!("An Option<Box<Term>> was None! This should not happen."),
    }
}

pub mod collection_of_const_funcs;
pub mod displaying;
pub mod interpreting;
