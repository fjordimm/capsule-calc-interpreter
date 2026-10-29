#![allow(non_snake_case)]

use crate::capsule_calc::{Constant, Term};

pub fn INCR() -> Term {
    Term::constant(Constant::func("INCR", |inp| match inp {
        Constant::Int(value) => Term::constant(Constant::int(*value + 1)),
        _ => Term::error(),
    }))
}
