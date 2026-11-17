use std::{collections::HashMap, mem};

use crate::capsule_calc::{
    ConstFuncBinary, ConstFuncUnary, Environment, Term, Variable, c_error, c_unit,
    interpreting::{is_irreducible, replace_var},
    some_or_panic, some_or_panic_r, some_or_panic_rm, t_const,
};

/// WARNING: Won't necessarily evaluate correctly.
pub fn take_step_lazy(term: &mut Box<Term>, environment: &mut Environment) {
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
                take_step_lazy(t1_s, environment);
            } else if !is_irreducible(t2_s) {
                take_step_lazy(t2_s, environment);
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
                take_step_lazy(value_s, environment);
            } else {
                let value_substed = mem::replace(value, None);

                environment.add(var.clone(), some_or_panic(value_substed));
                **term = t_const(c_unit());
            }
        }
    }
}

struct LazyVars(HashMap<Variable, Box<Term>>);
