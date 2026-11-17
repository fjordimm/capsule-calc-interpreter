use std::{collections::HashMap, hash::Hash};

use crate::capsule_calc::{
    collection_of_const_funcs::TO_CHURCH_BOOL,
    interpreting::{is_irreducible, take_step_lazy},
};

pub enum Term {
    Const(Const),
    ConstFuncUnary(ConstFuncUnary),
    ConstFuncBinary(ConstFuncBinary),
    Var(Var),
    Lambda(Var, Option<Box<Term>>),
    Application(Option<Box<Term>>, Option<Box<Term>>),
    Assignment(Var, Option<Box<Term>>),
}

pub fn t_const(value: Const) -> Term {
    Term::Const(value)
}

pub fn t_constfunc_unary(func: ConstFuncUnary) -> Term {
    Term::ConstFuncUnary(func)
}

pub fn t_constfunc_binary(func: ConstFuncBinary) -> Term {
    Term::ConstFuncBinary(func)
}

pub fn t_var(var: Var) -> Term {
    Term::Var(var)
}

pub fn t_lambda(param: Var, body: Term) -> Term {
    Term::Lambda(param, Some(Box::new(body)))
}

pub fn t_application(t1: Term, t2: Term) -> Term {
    Term::Application(Some(Box::new(t1)), Some(Box::new(t2)))
}

pub fn t_assignment(var: Var, value: Term) -> Term {
    Term::Assignment(var, Some(Box::new(value)))
}

pub fn tt_composition(t1: Term, t2: Term) -> Term {
    t_application(t_lambda(v_var("$_"), t2), t1)
}

pub fn tt_if_then_else(condition: Term, then_case: Term, else_case: Term) -> Term {
    t_application(
        t_application(
            t_application(
                t_application(t_constfunc_unary(TO_CHURCH_BOOL()), condition),
                t_lambda(v_var("$_"), then_case),
            ),
            t_lambda(v_var("$_"), else_case),
        ),
        t_const(c_unit()),
    )
}

pub fn tt_while_do(condition: Term, body: Term) -> Term {
    tt_letrec_be_in(
        v_var("$w"),
        t_lambda(
            v_var("$x"),
            tt_if_then_else(
                condition,
                tt_composition(body, t_application(t_var(v_var("$w")), t_const(c_unit()))),
                t_const(c_unit()),
            ),
        ),
        t_application(t_var(v_var("$w")), t_const(c_unit())),
    )
}

pub fn tt_let_be_in(var: Var, value: Term, term: Term) -> Term {
    t_application(t_lambda(var, term), value)
}

pub fn tt_letrec_be_in(var: Var, value: Term, term: Term) -> Term {
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
            Self::ConstFuncUnary(f) => Self::ConstFuncUnary(f.clone()),
            Self::ConstFuncBinary(f) => Self::ConstFuncBinary(f.clone()),
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

#[derive(Clone)]
pub struct ConstFuncUnary {
    name: String,
    f: fn(&Const) -> Term,
}

#[derive(Clone)]
pub struct ConstFuncBinary {
    name: String,
    f: fn(&Const, &Const) -> Term,
}

pub fn constfunc_unary(name: impl Into<String>, f: fn(&Const) -> Term) -> ConstFuncUnary {
    ConstFuncUnary {
        name: name.into(),
        f,
    }
}

pub fn constfunc_binary(name: impl Into<String>, f: fn(&Const, &Const) -> Term) -> ConstFuncBinary {
    ConstFuncBinary {
        name: name.into(),
        f,
    }
}
impl ConstFuncUnary {
    pub fn as_str(&self) -> String {
        format!("`{}`", self.name)
    }
}

impl ConstFuncBinary {
    pub fn as_str(&self) -> String {
        format!("`{}`", self.name)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Var(String);

pub fn v_var(name: impl Into<String>) -> Var {
    Var(name.into())
}

impl Var {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// TODO: Is it fine that this is an unordered map? I think so...
pub struct Environment(HashMap<Var, Box<Term>>);

impl Environment {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with(mut self, variable: Var, value: Term) -> Self {
        self.add(variable, Box::new(value));
        self
    }

    pub fn add(&mut self, var: Var, value: Box<Term>) {
        self.0.insert(var, value);
    }

    pub fn lookup(&self, var: &Var) -> Option<&Box<Term>> {
        self.0.get(var)
    }

    pub fn remove(&mut self, var: &Var) {
        self.0.remove(var);
    }

    pub fn fresh_var(&self) -> Var {
        loop {
            let candidate = v_var(format!("${}", rand::random::<u64>()));
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
        }
    }

    pub fn fresh_var_based_on(&self, old: &Var) -> Var {
        let mut candidate = v_var(format!("{}'", old.as_str()));
        loop {
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
            candidate = v_var(format!("{}'", candidate.as_str()));
        }
    }
}

/// Value of the HashMap represents the pair (the term, if it's irreducible or not).
pub struct LazyEnvironment(HashMap<Var, (Box<Term>, bool)>);

impl LazyEnvironment {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, var: Var, value: Box<Term>) {
        self.0.insert(var, (value, false));
    }

    pub fn lookup_without_eval(&self, var: &Var) -> Option<&(Box<Term>, bool)> {
        self.0.get(var)
    }

    pub fn lookup_and_eval(&mut self, var: &Var) -> LazyEnvironmentResult<'_> {
        match self.0.get(var) {
            None => LazyEnvironmentResult::Error,
            Some((value, _)) => {
                if is_irreducible(value.as_ref()) {
                    if let Some((value, is_irred)) = self.0.get_mut(var) {
                        *is_irred = true;
                        LazyEnvironmentResult::Irreducible(value)
                    } else {
                        panic!("This shouldn't happen.")
                    }
                } else {
                    if let Some((mut value, _)) = self.0.remove(var) {
                        take_step_lazy(&mut value, self);
                        self.0.insert(var.clone(), (value, false));

                        LazyEnvironmentResult::Reducible
                    } else {
                        panic!("This shouldn't happen.")
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, var: &Var) {
        self.0.remove(var);
    }

    pub fn fresh_var(&self) -> Var {
        loop {
            let candidate = v_var(format!("${}", rand::random::<u64>()));
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
        }
    }

    pub fn fresh_var_based_on(&self, old: &Var) -> Var {
        let mut candidate = v_var(format!("{}'", old.as_str()));
        loop {
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
            candidate = v_var(format!("{}'", candidate.as_str()));
        }
    }
}

pub enum LazyEnvironmentResult<'a> {
    Error,
    Reducible,
    Irreducible(&'a Box<Term>),
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
