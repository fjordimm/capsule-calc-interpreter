use crate::capsule_calc::{Constant, Term};

pub fn incr() -> Term {
    Term::constant(Constant::func("INCR", |inp| match inp {
        Constant::Int(value) => Term::constant(Constant::int(*value + 1)),
        _ => Term::error(),
    }))
}
