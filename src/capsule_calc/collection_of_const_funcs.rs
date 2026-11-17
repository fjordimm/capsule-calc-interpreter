#![allow(non_snake_case)]

use crate::capsule_calc::{
    Const, ConstFuncUnary, ConstFuncBinary, c_bool, c_error, c_int, constfunc_unary, constfunc_binary, t_const,
    t_lambda, t_var, var,
};

pub fn INCR() -> ConstFuncUnary {
    constfunc_unary("INCR", |inp| match inp {
        Const::Int(value) => t_const(c_int(*value + 1)),
        _ => t_const(c_error()),
    })
}

pub fn DECR() -> ConstFuncUnary {
    constfunc_unary("DECR", |inp| match inp {
        Const::Int(value) => t_const(c_int((*value - 1).max(0))),
        _ => t_const(c_error()),
    })
}

pub fn ADD() -> ConstFuncBinary {
    constfunc_binary("ADD", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(*value1 + *value2)),
        _ => t_const(c_error()),
    })
}

pub fn SUB() -> ConstFuncBinary {
    constfunc_binary("SUB", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(if *value2 > *value1 {
            0
        } else {
            *value1 - *value2
        })),
        _ => t_const(c_error()),
    })
}

pub fn MUL() -> ConstFuncBinary {
    constfunc_binary("MUL", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_int(*value1 * *value2)),
        _ => t_const(c_error()),
    })
}

pub fn DIV() -> ConstFuncBinary {
    constfunc_binary("DIV", |inp1, inp2| match (inp1, inp2) {
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
pub fn EQ() -> ConstFuncBinary {
    constfunc_binary("EQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Unit, Const::Unit) => t_const(c_bool(true)),
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 == *value2)),
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 == *value2)),
        _ => t_const(c_error()),
    })
}

/// Works for Const::Unit, Const::Bool, or Const::Int. And only works for values of the same type.
pub fn NEQ() -> ConstFuncBinary {
    constfunc_binary("NEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Unit, Const::Unit) => t_const(c_bool(false)),
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 != *value2)),
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 != *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LT() -> ConstFuncBinary {
    constfunc_binary("LT", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 < *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GT() -> ConstFuncBinary {
    constfunc_binary("GT", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 > *value2)),
        _ => t_const(c_error()),
    })
}

pub fn LEQ() -> ConstFuncBinary {
    constfunc_binary("LEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 <= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn GEQ() -> ConstFuncBinary {
    constfunc_binary("GEQ", |inp1, inp2| match (inp1, inp2) {
        (Const::Int(value1), Const::Int(value2)) => t_const(c_bool(*value1 >= *value2)),
        _ => t_const(c_error()),
    })
}

pub fn NOT() -> ConstFuncUnary {
    constfunc_unary("NOT", |inp| match inp {
        Const::Bool(value) => t_const(c_bool(!*value)),
        _ => t_const(c_error()),
    })
}

pub fn AND() -> ConstFuncBinary {
    constfunc_binary("AND", |inp1, inp2| match (inp1, inp2) {
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 && *value2)),
        _ => t_const(c_error()),
    })
}

pub fn OR() -> ConstFuncBinary {
    constfunc_binary("OR", |inp1, inp2| match (inp1, inp2) {
        (Const::Bool(value1), Const::Bool(value2)) => t_const(c_bool(*value1 || *value2)),
        _ => t_const(c_error()),
    })
}

pub fn TO_CHURCH_BOOL() -> ConstFuncUnary {
    constfunc_unary("TO_CHURCH_BOOL", |inp| match inp {
        Const::Bool(true) => t_lambda(
            var("$first"),
            t_lambda(var("$second"), t_var(var("$first"))),
        ),
        Const::Bool(false) => t_lambda(
            var("$first"),
            t_lambda(var("$second"), t_var(var("$second"))),
        ),
        _ => t_const(c_error()),
    })
}
