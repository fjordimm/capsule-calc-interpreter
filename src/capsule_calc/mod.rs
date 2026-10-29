use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum Term {
    Error,
    Constant(Constant),
    Var(Var),
    Lambda(Var, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Assignment(Var, Box<Term>),
    Composition(Box<Term>, Box<Term>),
    IfThenElse(Box<Term>, Box<Term>, Box<Term>),
    RepeatUntil(Box<Term>, Box<Term>),
}

impl Term {
    pub fn error() -> Self {
        Self::Error
    }

    pub fn constant(value: Constant) -> Self {
        Self::Constant(value)
    }

    pub fn var(v: Var) -> Self {
        Self::Var(v)
    }

    pub fn lambda(parameter: Var, body: Term) -> Self {
        Self::Lambda(parameter, Box::new(body))
    }

    pub fn application(function: Term, argument: Term) -> Self {
        Self::Application(Box::new(function), Box::new(argument))
    }

    pub fn assignment(variable: Var, value: Term) -> Self {
        Self::Assignment(variable, Box::new(value))
    }

    pub fn composition(first: Term, second: Term) -> Self {
        Self::Composition(Box::new(first), Box::new(second))
    }

    pub fn if_then_else(condition: Term, then_branch: Term, else_branch: Term) -> Self {
        Self::IfThenElse(
            Box::new(condition),
            Box::new(then_branch),
            Box::new(else_branch),
        )
    }

    pub fn repeat_until(body: Term, condition: Term) -> Self {
        Self::RepeatUntil(Box::new(body), Box::new(condition))
    }
}

/// Deep independent copy. Boxes clone their inner values, they do not copy over the pointer.
/// Exception: `ConstFunc`s clone over the box, they do not make a new one.
impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Self::Error => Self::Error,
            Self::Constant(c) => Self::Constant(c.clone()),
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
    pub fn none() -> Self {
        Self::None
    }

    pub fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    pub fn int(value: u32) -> Self {
        Self::Int(value)
    }

    pub fn func(name: impl Into<String>, f: fn(&Constant) -> Term) -> Self {
        Self::Func {
            name: name.into(),
            f: Box::new(f),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Constant::None => "None".to_owned(),
            Constant::Bool(value) => format!("Bool({})", value),
            Constant::Int(value) => format!("Int({})", value),
            Constant::Func { name, f: _ } => format!("Func({})", name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Var(String);

impl Var {
    pub fn var(name: impl Into<String>) -> Self {
        Self(name.into())
    }

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

    pub fn add(&mut self, variable: Var, value: Box<Term>) {
        self.0.insert(variable, value);
    }

    pub fn lookup(&self, variable: &Var) -> Option<&Box<Term>> {
        self.0.get(variable)
    }

    pub fn lookup_and_take(&mut self, _variable: &Var) -> Option<Box<Term>> {
        todo!()
    }

    pub fn fresh_var(&self) -> Var {
        loop {
            let candidate = Var::var(format!("FRESH_{}", rand::random::<u64>()));
            if !self.0.contains_key(&candidate) {
                return candidate;
            }
        }
    }
}

pub mod displaying;
pub mod interpreting;
pub mod some_funcs;
