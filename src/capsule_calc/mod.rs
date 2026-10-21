#[derive(Debug, Clone)]
pub enum Term {
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
    pub fn constant(value: Constant) -> Self {
        Self::Constant(value)
    }

    pub fn var(name: impl Into<String>) -> Self {
        Self::Var(Var::new(name))
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

#[derive(Debug, Clone)]
pub enum Constant {
    None,
    Bool(bool),
    Int(u32),
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
}

#[derive(Debug, Clone)]
pub struct Var(String);

impl Var {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    None,
    Bool,
    Int,
    Func(Box<Type>, Box<Type>),
}


