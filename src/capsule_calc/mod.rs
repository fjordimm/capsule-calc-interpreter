use std::{collections::HashMap, hash::Hash};

use crate::capsule_calc::collection_of_const_funcs::TO_CHURCH_BOOL;

pub enum Term {
    Const(Const),
    ConstFunc1(ConstFunc1),
    ConstFunc2(ConstFunc2),
    ConstFunc3(ConstFunc3),
    Var(Variable),
    Lambda(Variable, Option<Box<Term>>),
    Application(Option<Box<Term>>, Option<Box<Term>>),
    Assignment(Variable, Option<Box<Term>>),
}

pub fn t_const(value: Const) -> Term {
    Term::Const(value)
}

pub fn t_constfunc1(func: ConstFunc1) -> Term {
    Term::ConstFunc1(func)
}

pub fn t_constfunc2(func: ConstFunc2) -> Term {
    Term::ConstFunc2(func)
}

pub fn t_constfunc3(func: ConstFunc3) -> Term {
    Term::ConstFunc3(func)
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

pub fn tt_composition(t1: Term, t2: Term) -> Term {
    t_application(t_lambda(variable("$_"), t2), t1)
}

pub fn tt_if_then_else(condition: Term, then_case: Term, else_case: Term) -> Term {
    t_application(
        t_application(
            t_application(
                t_application(t_constfunc1(TO_CHURCH_BOOL()), condition),
                t_lambda(variable("$_"), then_case),
            ),
            t_lambda(variable("$_"), else_case),
        ),
        t_const(c_unit()),
    )
}

pub fn tt_while_do(condition: Term, body: Term) -> Term {
    tt_letrec_be_in(
        variable("$w"),
        t_lambda(
            variable("$x"),
            tt_if_then_else(
                condition,
                tt_composition(
                    body,
                    t_application(t_var(variable("$w")), t_const(c_unit())),
                ),
                t_const(c_unit()),
            ),
        ),
        t_application(t_var(variable("$w")), t_const(c_unit())),
    )
}

pub fn tt_let_be_in(var: Variable, value: Term, term: Term) -> Term {
    t_application(t_lambda(var, term), value)
}

pub fn tt_letrec_be_in(var: Variable, value: Term, term: Term) -> Term {
    t_application(
        t_lambda(var.clone(), tt_composition(t_assignment(var, value), term)),
        t_const(c_unit()),
    )
}

/// Deep independent copy. Boxes clone their inner values, they do not copy over the pointer.
impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Self::Const(c) => Self::Const(c.clone()),
            Self::ConstFunc1(f) => Self::ConstFunc1(f.clone()),
            Self::ConstFunc2(f) => Self::ConstFunc2(f.clone()),
            Self::ConstFunc3(f) => Self::ConstFunc3(f.clone()),
            Self::Var(v) => Self::Var(v.clone()),
            Self::Lambda(v, t) => Self::Lambda(v.clone(), t.to_owned()),
            Self::Application(t1, t2) => Self::Application(t1.to_owned(), t2.to_owned()),
            Self::Assignment(v, t) => Self::Assignment(v.clone(), t.to_owned()),
        }
    }
}

#[derive(Clone)]
pub enum Const {
    Error,
    Unit,
    Bool(bool),
    Int(u32),
}

pub fn c_error() -> Const {
    Const::Error
}

pub fn c_unit() -> Const {
    Const::Unit
}

pub fn c_bool(value: bool) -> Const {
    Const::Bool(value)
}

pub fn c_int(value: u32) -> Const {
    Const::Int(value)
}

impl Const {
    pub fn as_str(&self) -> String {
        match self {
            Const::Error => String::from("Error"),
            Const::Unit => String::from("Unit"),
            Const::Bool(value) => format!("Bool({})", value),
            Const::Int(value) => format!("Int({})", value),
        }
    }
}

/// Constant function with one argument.
#[derive(Clone)]
pub struct ConstFunc1 {
    name: String,
    f: fn(&Const) -> Term,
}

/// Constant function with two arguments.
#[derive(Clone)]
pub struct ConstFunc2 {
    name: String,
    f: fn(&Const, &Const) -> Term,
}

/// Constant function with three arguments.
#[derive(Clone)]
pub struct ConstFunc3 {
    name: String,
    f: fn(&Const, &Const, &Const) -> Term,
}

pub fn constfunc1(name: impl Into<String>, f: fn(&Const) -> Term) -> ConstFunc1 {
    ConstFunc1 {
        name: name.into(),
        f,
    }
}

pub fn constfunc2(name: impl Into<String>, f: fn(&Const, &Const) -> Term) -> ConstFunc2 {
    ConstFunc2 {
        name: name.into(),
        f,
    }
}

pub fn constfunc3(name: impl Into<String>, f: fn(&Const, &Const, &Const) -> Term) -> ConstFunc3 {
    ConstFunc3 {
        name: name.into(),
        f,
    }
}

impl ConstFunc1 {
    pub fn as_str(&self) -> String {
        format!("`{}`", self.name)
    }
}

impl ConstFunc2 {
    pub fn as_str(&self) -> String {
        format!("`{}`", self.name)
    }
}

impl ConstFunc3 {
    pub fn as_str(&self) -> String {
        format!("`{}`", self.name)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Variable(String);

pub fn variable(name: impl Into<String>) -> Variable {
    Variable(name.into())
}

impl Variable {
    pub fn as_str(&self) -> &str {
        &self.0
    }
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

    pub fn remove(&mut self, variable: &Variable) {
        self.0.remove(variable);
    }

    pub fn fresh_var(&self) -> Variable {
        loop {
            let candidate = variable(format!("${}", rand::random::<u64>()));
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
        }
    }

    pub fn fresh_var_based_on(&self, old: &Variable) -> Variable {
        let mut candidate = variable(format!("{}'", old.0));
        loop {
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
            candidate = variable(format!("{}'", candidate.0));
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
pub mod garbage_collection;
pub mod interpreting;
