use std::mem;

use crate::capsule_calc::{Constant, Environment, Term, Variable, t_error, t_var};

pub fn take_step(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Error => {}
        Term::Constant(_) => {}
        Term::Var(var) => {
            match environment.lookup(var) {
                None => **term = t_error(),
                Some(value) => *term = value.clone(), // TODO: this is not lazy.
            };
        }
        Term::Lambda(_, _) => {}
        Term::Application(t1, t2) => {
            if !is_irreducible(&t1) {
                take_step(t1, environment);
            } else if !is_irreducible(&t2) {
                take_step(t2, environment);
            } else {
                match (t1.as_mut(), t2.as_mut()) {
                    (Term::Constant(Constant::Func { name: _, f }), Term::Constant(c)) => {
                        **term = f(c)
                    }
                    (Term::Lambda(param, body), _) => {
                        let fresh_var = environment.fresh_var();

                        let mut substed_inner = mem::replace(body, Box::new(Term::Error)); // TODO: using mem::replace so I can own it, but might not be the most efficient.
                        replace_var(&mut substed_inner, param, &fresh_var);

                        let t2 = mem::replace(t2, Box::new(Term::Error)); // TODO: using mem::replace so I can own it, but might not be the most efficient.

                        *term = substed_inner;
                        environment.add(fresh_var, t2);
                    }
                    _ => {
                        println!("Invalid application.");
                        **term = Term::Error;
                    }
                }
            }
        }
        _ => todo!(),
    }
}

pub fn is_irreducible(term: &Term) -> bool {
    match term {
        Term::Error => true,
        Term::Constant(_) => true,
        Term::Lambda(_, _) => true,
        _ => false,
    }
}

/// Assumes `new` is a fresh variable.
fn replace_var(term: &mut Term, original: &Variable, new: &Variable) {
    match term {
        Term::Error => {}
        Term::Constant(_) => {}
        Term::Var(var) => {
            if var == original {
                *term = t_var(new.clone());
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
            // TODO: is this correct?

            if var == original {
                *var = new.clone();
            }

            replace_var(value, original, new);
        }
        Term::Composition(t1, t2) => {
            replace_var(t1, original, new);
            replace_var(t2, original, new);
        }
        Term::IfThenElse(condition, then_branch, else_branch) => {
            replace_var(condition, original, new);
            replace_var(then_branch, original, new);
            replace_var(else_branch, original, new);
        }
        Term::RepeatUntil(body, condition) => {
            replace_var(body, original, new);
            replace_var(condition, original, new);
        }
    }
}
