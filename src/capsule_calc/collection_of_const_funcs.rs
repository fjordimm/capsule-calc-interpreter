#![allow(non_snake_case)]

use crate::capsule_calc::{
    Constant, c_bool, c_error, c_func1, c_func2, c_int, t_const, t_lambda, t_var, variable,
};

pub fn INCR() -> Constant {
    c_func1("INCR", |inp| match inp {
        Constant::Int(value) => t_const(c_int(*value + 1)),
        _ => t_const(c_error()),
    })
}

pub fn DECR() -> Constant {
    c_func1("DECR", |inp| match inp {
        Constant::Int(value) => t_const(c_int((*value - 1).max(0))),
        _ => t_const(c_error()),
    })
}

pub fn ADD() -> Constant {
    c_func2("ADD", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_int(*value1 + *value2)),
        _ => t_const(c_error()),
    })
}

pub fn SUB() -> Constant {
    c_func2("SUB", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_int(if *value2 > *value1 {
            0
        } else {
            *value1 - *value2
        })),
        _ => t_const(c_error()),
    })
}

pub fn MUL() -> Constant {
    c_func2("MUL", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_int(*value1 * *value2)),
        _ => t_const(c_error()),
    })
}

pub fn DIV() -> Constant {
    c_func2("DIV", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => {
            if *value2 == 0 {
                t_const(c_error())
            } else {
                t_const(c_int(*value1 / *value2))
            }
        }
        _ => t_const(c_error()),
    })
}

/// Works for Constant::Unit, Constant::Bool, or Constant::Int. And only works for values of the same type.
pub fn EQ() -> Constant {
    c_func2("EQ", |inp1, inp2| match (inp1, inp2) {
        (Constant::Unit, Constant::Unit) => t_const(c_bool(true)),
        (Constant::Bool(value1), Constant::Bool(value2)) => t_const(c_bool(*value1 == *value2)),
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 == *value2)),
        _ => t_const(c_error()),
    })
}

/// Works for Constant::Unit, Constant::Bool, or Constant::Int. And only works for values of the same type.
pub fn NEQ() -> Constant {
    c_func2("NEQ", |inp1, inp2| match (inp1, inp2) {
        (Constant::Unit, Constant::Unit) => t_const(c_bool(false)),
        (Constant::Bool(value1), Constant::Bool(value2)) => t_const(c_bool(*value1 != *value2)),
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 != *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LT() -> Constant {
    c_func2("LT", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 < *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GT() -> Constant {
    c_func2("GT", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 > *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LEQ() -> Constant {
    c_func2("LEQ", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 <= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GEQ() -> Constant {
    c_func2("GEQ", |inp1, inp2| match (inp1, inp2) {
        (Constant::Int(value1), Constant::Int(value2)) => t_const(c_bool(*value1 >= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn NOT() -> Constant {
    c_func1("NOT", |inp| match inp {
        Constant::Bool(value) => t_const(c_bool(!*value)),
        _ => t_const(c_error()),
    })
}

pub fn AND() -> Constant {
    c_func2("AND", |inp1, inp2| match (inp1, inp2) {
        (Constant::Bool(value1), Constant::Bool(value2)) => t_const(c_bool(*value1 && *value2)),
        _ => t_const(c_error()),
    })
}

pub fn OR() -> Constant {
    c_func2("OR", |inp1, inp2| match (inp1, inp2) {
        (Constant::Bool(value1), Constant::Bool(value2)) => t_const(c_bool(*value1 || *value2)),
        _ => t_const(c_error()),
    })
}

pub fn TO_CHURCH_BOOL() -> Constant {
    c_func1("TO_CHURCH_BOOL", |inp| match inp {
        Constant::Bool(true) => t_lambda(
            variable("$first"),
            t_lambda(variable("$second"), t_var(variable("$first"))),
        ),
        Constant::Bool(false) => t_lambda(
            variable("$first"),
            t_lambda(variable("$second"), t_var(variable("$second"))),
        ),
        _ => t_const(c_error()),
    })
}
