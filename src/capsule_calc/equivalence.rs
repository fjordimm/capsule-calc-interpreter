use crate::capsule_calc::{Const, ConstFuncUnary, Term};

pub enum TermEquivalenceResult {
    Ok(bool),
    EncounteredVar,
}

/// Uses De Bruijn indices (in reverse order) to check equivalence. E.g. `\x -> x` is equivalent to `\y -> y`
pub fn check_terms_equivalent(a: &Term, b: &Term) -> TermEquivalenceResult {
    match (a, b) {
        (Term::Const(a_const), Term::Const(b_const)) => check_const_equivalent(a_const, b_const),
        _ => TermEquivalenceResult::Ok(false),
    }
}

fn check_const_equivalent(a: &Const, b: &Const) -> TermEquivalenceResult {
    match (a, b) {
        (Const::Error, Const::Error) => TermEquivalenceResult::Ok(true),
        (Const::Unit, Const::Unit) => TermEquivalenceResult::Ok(true),
        (Const::Bool(a_val), Const::Bool(b_val)) => TermEquivalenceResult::Ok(*a_val == *b_val),
        (Const::Int(a_val), Const::Int(b_val)) => TermEquivalenceResult::Ok(*a_val == *b_val),
        _ => TermEquivalenceResult::Ok(false),
    }
}

fn check_const_func_unary_equivalent(
    a: &ConstFuncUnary,
    b: &ConstFuncUnary,
) -> TermEquivalenceResult {
}
