#![allow(non_snake_case)]

use crate::capsule_calc::{Constant, Term, c_func, c_int, t_constant, t_error};

pub fn INCR() -> Term {
    t_constant(c_func("INCR", |inp| match inp {
        Constant::Int(value) => t_constant(c_int(*value + 1)),
        _ => t_error(),
    }))
}
