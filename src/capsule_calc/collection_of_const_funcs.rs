#![allow(non_snake_case)]

use crate::capsule_calc::{
    Const, ConstFunc1, ConstFunc2, c_bool, c_error, c_int, constfunc1, constfunc2, t_const,
    t_lambda, t_var, variable,
};

pub fn INCR() -> ConstFunc1 {
    constfunc1("INCR", |inp| match inp {
        Const::Int(value) => t_const(c_int(*value + 1)),
        _ => t_const(c_error()),
    })
}

pub fn DECR() -> ConstFunc1 {
    constfunc1("DECR", |inp| match inp {
        Const::Int(value) => t_const(c_int((*value - 1).max(0))),
        _ => t_const(c_error()),
    })
}

pub fn ADD() -> ConstFunc2 {
    constfunc2("ADD", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(*value1 + *value2)),
        _ => t_const(c_error()),
    })
}

pub fn SUB() -> ConstFunc2 {
    constfunc2("SUB", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(if *value2 > *value1 {
            0
        } else {
            *value1 - *value2
        })),
        _ => t_const(c_error()),
    })
}

pub fn MUL() -> ConstFunc2 {
    constfunc2("MUL", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(*value1 * *value2)),
        _ => t_const(c_error()),
    })
}

pub fn DIV() -> ConstFunc2 {
    constfunc2("DIV", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => {
            if *value2 == 0 {
                t_const(c_error())
            } else {
                t_const(c_int(*value1 / *value2))
            }
        }
        _ => t_const(c_error()),
    })
}

/// Works for Const::Unit, Const::Bool, or Const::Int. And only works for values of the same type.
pub fn EQ() -> ConstFunc2 {
    constfunc2("EQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Unit, Const::Unit) => t_const(c_bool(true)),
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 == *value2)),
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 == *value2)),
        _ => t_const(c_error()),
    })
}

/// Works for Const::Unit, Const::Bool, or Const::Int. And only works for values of the same type.
pub fn NEQ() -> ConstFunc2 {
    constfunc2("NEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Unit, Const::Unit) => t_const(c_bool(false)),
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 != *value2)),
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 != *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LT() -> ConstFunc2 {
    constfunc2("LT", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 < *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GT() -> ConstFunc2 {
    constfunc2("GT", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 > *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LEQ() -> ConstFunc2 {
    constfunc2("LEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 <= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GEQ() -> ConstFunc2 {
    constfunc2("GEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 >= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn NOT() -> ConstFunc1 {
    constfunc1("NOT", |inp| match inp {
        Const::Bool(value) => t_const(c_bool(!*value)),
        _ => t_const(c_error()),
    })
}

pub fn AND() -> ConstFunc2 {
    constfunc2("AND", |inp1, inp2| match (inp1, inp2) {
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 && *value2)),
        _ => t_const(c_error()),
    })
}

pub fn OR() -> ConstFunc2 {
    constfunc2("OR", |inp1, inp2| match (inp1, inp2) {
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 || *value2)),
        _ => t_const(c_error()),
    })
}

pub fn TO_CHURCH_BOOL() -> ConstFunc1 {
    constfunc1("TO_CHURCH_BOOL", |inp| match inp {
        Const::Bool(true) => t_lambda(
            variable("$first"),
            t_lambda(variable("$second"), t_var(variable("$first"))),
        ),
        Const::Bool(false) => t_lambda(
            variable("$first"),
            t_lambda(variable("$second"), t_var(variable("$second"))),
        ),
        _ => t_const(c_error()),
    })
}
