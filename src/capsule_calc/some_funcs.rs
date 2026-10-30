#![allow(non_snake_case)]

use crate::capsule_calc::{Constant, c_bool, c_func, c_int, t_const, t_error, t_lambda, variable};

pub fn INCR() -> Constant {
    c_func("INCR", |inp| match inp {
        Constant::Int(value) => t_const(c_int(*value + 1)),
        _ => t_error(),
    })
}

pub fn NOT() -> Constant {
    c_func("NOT", |inp| match inp {
        Constant::Bool(value) => t_const(c_bool(!*value)),
        _ => t_error(),
    })
}

pub fn IS_5() -> Constant {
    c_func("IS_5", |inp| match inp {
        Constant::Int(value) => {
            if *value == 5 {
                t_const(c_bool(true))
            } else {
                t_const(c_bool(false))
            }
        }
        _ => t_error(),
    })
}
