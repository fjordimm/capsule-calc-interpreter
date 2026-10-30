use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum Term {
    Error,
    Const(Constant),
    Var(Variable),
    Lambda(Variable, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Assignment(Variable, Box<Term>),
    Composition(Box<Term>, Box<Term>),
    IfThenElse(Box<Term>, Box<Term>, Box<Term>),
    RepeatUntil(Box<Term>, Box<Term>),
}

pub fn t_error() -> Term {
    Term::Error
}

pub fn t_const(value: Constant) -> Term {
    Term::Const(value)
}

pub fn t_var(var: Variable) -> Term {
    Term::Var(var)
}

pub fn t_lambda(param: Variable, body: Term) -> Term {
    Term::Lambda(param, Box::new(body))
}

pub fn t_application(t1: Term, t2: Term) -> Term {
    Term::Application(Box::new(t1), Box::new(t2))
}

pub fn t_assignment(var: Variable, value: Term) -> Term {
    Term::Assignment(var, Box::new(value))
}

pub fn t_composition(t1: Term, t2: Term) -> Term {
    Term::Composition(Box::new(t1), Box::new(t2))
}

pub fn t_if_then_else(condition: Term, then_branch: Term, else_branch: Term) -> Term {
    Term::IfThenElse(
        Box::new(condition),
        Box::new(then_branch),
        Box::new(else_branch),
    )
}

pub fn t_repeat_until(body: Term, condition: Term) -> Term {
    Term::RepeatUntil(Box::new(body), Box::new(condition))
}

/// Deep independent copy. Boxes clone their inner values, they do not copy over the pointer.
/// Exception: `ConstFunc`s clone over the box, they do not make a new one.
impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Self::Error => Self::Error,
            Self::Const(c) => Self::Const(c.clone()),
            Self::Var(v) => Self::Var(v.clone()),
            Self::Lambda(v, t) => Self::Lambda(v.clone(), t.to_owned()),
            Self::Application(t1, t2) => Self::Application(t1.to_owned(), t2.to_owned()),
            Self::Assignment(v, t) => Self::Assignment(v.clone(), t.to_owned()),
            Self::Composition(t1, t2) => Self::Composition(t1.to_owned(), t2.to_owned()),
            Self::IfThenElse(t1, t2, t3) => {
                Self::IfThenElse(t1.to_owned(), t2.to_owned(), t3.to_owned())
            }
            Self::RepeatUntil(t1, t2) => Self::RepeatUntil(t1.to_owned(), t2.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    None,
    Bool(bool),
    Int(u32),
    Func {
        name: String,
        f: Box<fn(&Constant) -> Term>,
    },
}

impl Constant {
    pub fn as_str(&self) -> String {
        match self {
            Constant::None => "None".to_owned(),
            Constant::Bool(value) => format!("Bool({})", value),
            Constant::Int(value) => format!("Int({})", value),
            Constant::Func { name, f: _ } => format!("Func({})", name),
        }
    }
}

pub fn c_none() -> Constant {
    Constant::None
}

pub fn c_bool(value: bool) -> Constant {
    Constant::Bool(value)
}

pub fn c_int(value: u32) -> Constant {
    Constant::Int(value)
}

pub fn c_func(name: impl Into<String>, f: fn(&Constant) -> Term) -> Constant {
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

pub mod displaying;
pub mod interpreting;
pub mod some_funcs;
