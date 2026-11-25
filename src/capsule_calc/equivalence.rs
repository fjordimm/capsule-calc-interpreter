use std::collections::HashMap;

use crate::capsule_calc::{Const, ConstFuncBinary, ConstFuncUnary, Term, Var};

#[derive(Debug)]
pub enum TermEquivalenceResult {
    Ok(bool),
    EncounteredFreeVar,
    /// Other errors, for instance finding an inner term as a None.
    Error,
}

/// Although it works for reducible expressions, they might not be equivalent unless they are fully reduced.
/// E.g. `(\x -> x) 5` is not equivalent to `5`.
///
/// Bound variable names do not need to be equivalent for the term to be equivalent.
/// E.g. `\x -> x` is equivalent to `\y -> y`.
/// Does this by using De Bruijn indices (in reverse order).
///
/// Any encounter with a free variable (whether it has a value in the environment or not) will return `TermEquivalenceResult::EncounteredFreeVar`.
pub fn check_terms_equivalent(a: &Term, b: &Term) -> TermEquivalenceResult {
    // TODO: could be much more efficient, since I'm cloning the hashmap a lot.
    check_terms_equivalent_h(a, b, 0, HashMap::new(), HashMap::new())
}

pub fn check_terms_equivalent_h(
    a: &Term,
    b: &Term,
    de_bruijn_index: u32,
    a_var_numberings: HashMap<Var, u32>,
    b_var_numberings: HashMap<Var, u32>,
) -> TermEquivalenceResult {
    match (a, b) {
        (Term::Const(a_const), Term::Const(b_const)) => check_const_equivalent(a_const, b_const),
        (Term::ConstFuncUnary(a_func), Term::ConstFuncUnary(b_func)) => {
            check_const_func_unary_equivalent(a_func, b_func)
        }
        (Term::ConstFuncBinary(a_func), Term::ConstFuncBinary(b_func)) => {
            check_const_func_binary_equivalent(a_func, b_func)
        }
        (Term::Var(a_var), Term::Var(b_var)) => {
            match (a_var_numberings.get(a_var), a_var_numberings.get(b_var)) {
                (Some(a_var_num), Some(b_var_num)) => {
                    TermEquivalenceResult::Ok(*a_var_num == *b_var_num)
                }
                _ => TermEquivalenceResult::EncounteredFreeVar,
            }
        }
        (Term::Lambda(a_binder, a_body), Term::Lambda(b_binder, b_body)) => {
            match (a_body, b_body) {
                (Some(a_body), Some(b_body)) => {
                    let mut new_a_var_numberings = a_var_numberings;
                    new_a_var_numberings.insert(a_binder.clone(), de_bruijn_index);

                    let mut new_b_var_numberings = b_var_numberings;
                    new_b_var_numberings.insert(b_binder.clone(), de_bruijn_index);

                    check_terms_equivalent_h(
                        a_body,
                        b_body,
                        de_bruijn_index + 1,
                        new_a_var_numberings,
                        new_b_var_numberings,
                    )
                }
                _ => TermEquivalenceResult::Error,
            }
        }
        (Term::Application(a_t1, a_t2), Term::Application(b_t1, b_t2)) => {
            match (a_t1, a_t2, b_t1, b_t2) {
                (Some(a_t1), Some(a_t2), Some(b_t1), Some(b_t2)) => {
                    match check_terms_equivalent_h(
                        a_t1,
                        b_t1,
                        de_bruijn_index,
                        a_var_numberings.clone(),
                        b_var_numberings.clone(),
                    ) {
                        TermEquivalenceResult::Ok(t1_eq) => match check_terms_equivalent_h(
                            a_t2,
                            b_t2,
                            de_bruijn_index,
                            a_var_numberings,
                            b_var_numberings,
                        ) {
                            TermEquivalenceResult::Ok(t2_eq) => {
                                TermEquivalenceResult::Ok(t1_eq && t2_eq)
                            }
                            other => other,
                        },
                        other => other,
                    }
                }
                _ => TermEquivalenceResult::Error,
            }
        }
        (Term::Assignment(a_lhs, a_rhs), Term::Assignment(b_lhs, b_rhs)) => match (a_rhs, b_rhs) {
            (Some(a_rhs), Some(b_rhs)) => {
                match check_terms_equivalent_h(
                    a_rhs,
                    b_rhs,
                    de_bruijn_index,
                    a_var_numberings,
                    b_var_numberings,
                ) {
                    TermEquivalenceResult::Ok(rhs_eq) => {
                        TermEquivalenceResult::Ok((*a_lhs == *b_lhs) && rhs_eq)
                    }
                    other => other,
                }
            }
            _ => TermEquivalenceResult::Error,
        },
        (Term::IfThenElse(a_t1, a_t2, a_t3), Term::IfThenElse(b_t1, b_t2, b_t3)) => {
            match (a_t1, a_t2, a_t3, b_t1, b_t2, b_t3) {
                (Some(a_t1), Some(a_t2), Some(a_t3), Some(b_t1), Some(b_t2), Some(b_t3)) => {
                    match check_terms_equivalent_h(
                        a_t1,
                        b_t1,
                        de_bruijn_index,
                        a_var_numberings.clone(),
                        b_var_numberings.clone(),
                    ) {
                        TermEquivalenceResult::Ok(t1_eq) => match check_terms_equivalent_h(
                            a_t2,
                            b_t2,
                            de_bruijn_index,
                            a_var_numberings.clone(),
                            b_var_numberings.clone(),
                        ) {
                            TermEquivalenceResult::Ok(t2_eq) => match check_terms_equivalent_h(
                                a_t3,
                                b_t3,
                                de_bruijn_index,
                                a_var_numberings,
                                b_var_numberings,
                            ) {
                                TermEquivalenceResult::Ok(t3_eq) => {
                                    TermEquivalenceResult::Ok(t1_eq && t2_eq && t3_eq)
                                }
                                other => other,
                            },
                            other => other,
                        },
                        other => other,
                    }
                }
                _ => TermEquivalenceResult::Error,
            }
        }
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
    TermEquivalenceResult::Ok(std::ptr::fn_addr_eq(a.f, b.f))
}

fn check_const_func_binary_equivalent(
    a: &ConstFuncBinary,
    b: &ConstFuncBinary,
) -> TermEquivalenceResult {
    TermEquivalenceResult::Ok(std::ptr::fn_addr_eq(a.f, b.f))
}
