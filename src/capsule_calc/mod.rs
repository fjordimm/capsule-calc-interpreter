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

#[derive(Debug, Clone)]
pub enum Constant {
    None,
    Bool(bool),
    Int(u32),
}

#[derive(Debug, Clone)]
pub struct Var(String);

#[derive(Debug, Clone)]
pub enum Type {
    None,
    Bool,
    Int,
    Func(Box<Type>, Box<Type>),
}
