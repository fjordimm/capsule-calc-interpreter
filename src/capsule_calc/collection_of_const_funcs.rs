#![allow(non_snake_case)]

use crate::capsule_calc::{
    Constant, c_bool, c_error, c_func, c_int, t_const, t_lambda, t_var, variable,
};

pub fn INCR() -> Constant {
    c_func("INCR", |inp| match inp {
        Constant::Int(value) => t_const(c_int(*value + 1)),
        _ => t_const(c_error()),
    })
}

pub fn DECR() -> Constant {
    c_func("DECR", |inp| match inp {
        Constant::Int(value) => t_const(c_int((*value - 1).max(0))),
        _ => t_const(c_error()),
    })
}

pub fn ADD() -> Constant {
    c_func("ADD", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("ADD-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_int(value1_ + *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn SUB() -> Constant {
    c_func("SUB", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("SUB-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => {
                    if *value2 > value1_ {
                        t_const(c_int(0))
                    } else {
                        t_const(c_int(value1_ - *value2))
                    }
                }
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn MUL() -> Constant {
    c_func("MUL", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("MUL-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_int(value1_ * *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn DIV() -> Constant {
    c_func("DIV", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("DIV-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => {
                    if *value2 == 0 {
                        t_const(c_error())
                    } else {
                        t_const(c_int(value1_ / *value2))
                    }
                }
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn EQ() -> Constant {
    c_func("EQ", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("EQ-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ == *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn NEQ() -> Constant {
    c_func("NEQ", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("NEQ-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ != *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn LT() -> Constant {
    c_func("LT", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("LT-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ < *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn GT() -> Constant {
    c_func("GT", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("GT-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ > *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn LEQ() -> Constant {
    c_func("LEQ", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("LEQ-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ <= *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn GEQ() -> Constant {
    c_func("GEQ", |inp1| match inp1 {
        Constant::Int(value1) => t_const(c_func("GEQ-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Int(value2) => t_const(c_bool(value1_ >= *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn NOT() -> Constant {
    c_func("NOT", |inp| match inp {
        Constant::Bool(value) => t_const(c_bool(!*value)),
        _ => t_const(c_error()),
    })
}

pub fn AND() -> Constant {
    c_func("AND", |inp1| match inp1 {
        Constant::Bool(value1) => t_const(c_func("AND-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Bool(value2) => t_const(c_bool(value1_ && *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn OR() -> Constant {
    c_func("OR", |inp1| match inp1 {
        Constant::Bool(value1) => t_const(c_func("OR-closure", {
            let value1_ = *value1;
            move |inp2| match inp2 {
                Constant::Bool(value2) => t_const(c_bool(value1_ || *value2)),
                _ => t_const(c_error()),
            }
        })),
        _ => t_const(c_error()),
    })
}

pub fn IS_0() -> Constant {
    c_func("IS_0", |inp| match inp {
        Constant::Int(value) => {
            if *value == 0 {
                t_const(c_bool(true))
            } else {
                t_const(c_bool(false))
            }
        }
        _ => t_const(c_error()),
    })
}

pub fn TO_CHURCH_BOOL() -> Constant {
    c_func("TO_CHURCH_BOOL", |inp| match inp {
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
