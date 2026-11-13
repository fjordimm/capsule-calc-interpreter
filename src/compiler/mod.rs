use crate::{
    capsule_calc::Term,
    compiler::{lexing::lex, parsing::parse},
};

pub fn compile(input: &str) -> Box<Term> {
    parse(&lex(&input))
}

pub mod lexing;
pub mod parsing;
