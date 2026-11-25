use std::mem;

use crate::capsule_calc::{
    Const, ConstFuncBinary, ConstFuncUnary, Environment, LazyEnvironmentResult, Term, Var, c_error,
    c_unit, some_or_panic, some_or_panic_r, some_or_panic_rm, t_const, t_var,
};

pub fn take_step(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Const(_) => {}
        Term::ConstFuncUnary(_) => {}
        Term::ConstFuncBinary(_) => {}
        Term::Var(var) => match environment.lookup(var) {
            None => **term = t_const(c_error()),
            Some(value) => *term = value.clone(),
        },
        Term::Lambda(_, _) => {}
        Term::Application(t1, t2) => {
            let t1_s = some_or_panic_rm(t1);
            let t2_s = some_or_panic_rm(t2);

            if !is_irreducible(t1_s) {
                take_step(t1_s, environment);
            } else if !is_irreducible(t2_s) {
                take_step(t2_s, environment);
            } else {
                match (t1_s.as_mut(), t2_s.as_mut()) {
                    (Term::ConstFuncUnary(ConstFuncUnary { name: _, f }), Term::Const(arg)) => {
                        **term = f(arg);
                    }
                    (Term::Application(it1, it2), Term::Const(arg2)) => {
                        let it1_s = some_or_panic_r(it1);
                        let it2_s = some_or_panic_r(it2);

                        if let (
                            Term::ConstFuncBinary(ConstFuncBinary { name: _, f }),
                            Term::Const(arg1),
                        ) = (it1_s.as_ref(), it2_s.as_ref())
                        {
                            **term = f(arg1, arg2);
                        } else {
                            panic!("Something went wrong with reducing an Application.");
                        }
                    }
                    (Term::Lambda(param, body), _) => {
                        let fresh_var = environment.fresh_var_based_on(param);

                        let mut body_substed = mem::replace(body, None);
                        replace_var(&mut body_substed, param, &fresh_var);

                        let t2_substed = mem::replace(t2, None);

                        environment.add_irreducible(fresh_var, some_or_panic(t2_substed));
                        *term = some_or_panic(body_substed);
                    }
                    _ => {
                        **term = t_const(c_error());
                    }
                }
            }
        }
        Term::Assignment(var, value) => {
            let value_s = some_or_panic_rm(value);

            if !is_irreducible(value_s) {
                take_step(value_s, environment);
            } else {
                let value_substed = mem::replace(value, None);

                environment.add_irreducible(var.clone(), some_or_panic(value_substed));
                **term = t_const(c_unit());
            }
        }
        Term::IfThenElse(condition, then_case, else_case) => {
            let condition_s = some_or_panic_rm(condition);

            if !is_irreducible(condition_s) {
                take_step(condition_s, environment);
            } else {
                match condition_s.as_ref() {
                    Term::Const(Const::Bool(condition_val)) => {
                        if *condition_val {
                            let then_case_substed = mem::replace(then_case, None);
                            *term = some_or_panic(then_case_substed);
                        } else {
                            let else_case_substed = mem::replace(else_case, None);
                            *term = some_or_panic(else_case_substed);
                        }
                    }
                    _ => **term = t_const(c_error()),
                }
            }
        }
    }
}

/// WARNING: Won't necessarily evaluate to the same thing as take_step.
pub fn take_step_lazy(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Const(_) => {}
        Term::ConstFuncUnary(_) => {}
        Term::ConstFuncBinary(_) => {}
        Term::Var(var) => match environment.lookup_or_step(var) {
            LazyEnvironmentResult::NotFound => **term = t_const(c_error()),
            LazyEnvironmentResult::Reducible => {}
            LazyEnvironmentResult::Irreducible(value) => *term = value.clone(),
        },
        Term::Lambda(_, _) => {}
        Term::Application(t1, t2) => {
            let t1_s = some_or_panic_rm(t1);
            let t2_s = some_or_panic_rm(t2);

            if !is_irreducible(t1_s) {
                take_step_lazy(t1_s, environment);
            } else {
                match t1_s.as_mut() {
                    Term::Lambda(param, body) => {
                        let fresh_var = environment.fresh_var_based_on(param);

                        let mut body_substed = mem::replace(body, None);
                        replace_var(&mut body_substed, param, &fresh_var);

                        let t2_substed = mem::replace(t2, None);

                        environment.add_possibly_reducible(fresh_var, some_or_panic(t2_substed));
                        *term = some_or_panic(body_substed);
                    }
                    t1_s => {
                        if !is_irreducible(t2_s) {
                            take_step(t2_s, environment);
                        } else {
                            match (t1_s, t2_s.as_mut()) {
                                (
                                    Term::ConstFuncUnary(ConstFuncUnary { name: _, f }),
                                    Term::Const(arg),
                                ) => {
                                    **term = f(arg);
                                }
                                (Term::Application(it1, it2), Term::Const(arg2)) => {
                                    let it1_s = some_or_panic_r(it1);
                                    let it2_s = some_or_panic_r(it2);

                                    if let (
                                        Term::ConstFuncBinary(ConstFuncBinary { name: _, f }),
                                        Term::Const(arg1),
                                    ) = (it1_s.as_ref(), it2_s.as_ref())
                                    {
                                        **term = f(arg1, arg2);
                                    } else {
                                        panic!(
                                            "Something went wrong with reducing an Application."
                                        );
                                    }
                                }
                                _ => **term = t_const(c_error()),
                            }
                        }
                    }
                }
            }
        }
        Term::Assignment(var, value) => {
            let value_substed = mem::replace(value, None);

            environment.add_possibly_reducible(var.clone(), some_or_panic(value_substed));
            **term = t_const(c_unit());
        }
        Term::IfThenElse(condition, then_case, else_case) => {
            let condition_s = some_or_panic_rm(condition);

            if !is_irreducible(condition_s) {
                take_step(condition_s, environment);
            } else {
                match condition_s.as_ref() {
                    Term::Const(Const::Bool(condition_val)) => {
                        if *condition_val {
                            let then_case_substed = mem::replace(then_case, None);
                            *term = some_or_panic(then_case_substed);
                        } else {
                            let else_case_substed = mem::replace(else_case, None);
                            *term = some_or_panic(else_case_substed);
                        }
                    }
                    _ => **term = t_const(c_error()),
                }
            }
        }
    }
}

pub fn is_irreducible(term: &Term) -> bool {
    match term {
        Term::Const(_) => true,
        Term::ConstFuncUnary(_) => true,
        Term::ConstFuncBinary(_) => true,
        Term::Lambda(_, _) => true,
        Term::Application(t1, t2) => match some_or_panic_r(t1).as_ref() {
            Term::ConstFuncBinary(_) => is_irreducible(some_or_panic_r(t2)),
            _ => false,
        },
        _ => false,
    }
}

/// Assumes `new` is a fresh variable.
pub(super) fn replace_var(term: &mut Option<Box<Term>>, original: &Var, new: &Var) {
    let term = some_or_panic_rm(term);

    match term.as_mut() {
        Term::Const(_) => {}
        Term::ConstFuncUnary(_) => {}
        Term::ConstFuncBinary(_) => {}
        Term::Var(var) => {
            if var == original {
                **term = t_var(new.clone());
            }
        }
        Term::Lambda(param, inner) => {
            if param != original {
                if param == new {
                    panic!(
                        "This should not happen. Somehow a lambda parameter had the same name as a fresh variable."
                    );
                    // TODO: you can avoid this entirely by making Var an enum.
                }

                replace_var(inner, original, new);
            }
        }
        Term::Application(t1, t2) => {
            replace_var(t1, original, new);
            replace_var(t2, original, new);
        }
        Term::Assignment(var, value) => {
            if var == original {
                *var = new.clone();
            }

            replace_var(value, original, new);
        }
        Term::IfThenElse(t1, t2, t3) => {
            replace_var(t1, original, new);
            replace_var(t2, original, new);
            replace_var(t3, original, new);
        }
    }
}
