use std::mem;

use crate::capsule_calc::{
    Constant, Environment, Term, Variable, c_error, c_unit, some_or_panic, some_or_panic_rm,
    t_const, t_var,
};

pub fn take_step(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Const(_) => {}
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
                    (Term::Const(Constant::Func { name: _, f }), Term::Const(c)) => **term = f(c),
                    (Term::Lambda(param, body), _) => {
                        let fresh_var = environment.fresh_var_based_on(param);

                        let mut body_substed = mem::replace(body, None);
                        replace_var(&mut body_substed, param, &fresh_var);

                        let t2_substed = mem::replace(t2, None);

                        environment.add(fresh_var, some_or_panic(t2_substed));
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

                environment.add(var.clone(), some_or_panic(value_substed));
                **term = t_const(c_unit());
            }
        }
    }
}

pub fn is_irreducible(term: &Term) -> bool {
    match term {
        Term::Const(_) => true,
        Term::Lambda(_, _) => true,
        _ => false,
    }
}

/// Assumes `new` is a fresh variable.
fn replace_var(term: &mut Option<Box<Term>>, original: &Variable, new: &Variable) {
    let term = some_or_panic_rm(term);

    match term.as_mut() {
        Term::Const(_) => {}
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
    }
}
