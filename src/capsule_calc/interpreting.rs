use std::mem;

use crate::capsule_calc::{Constant, Environment, Term, Var};

pub fn take_step(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Error => {}
        Term::Constant(_) => {}
        Term::Var(var) => {
            match environment.lookup(var) {
                None => **term = Term::error(),
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
                    (Term::Lambda(var, inner), _) => {
                        let fresh_var = environment.fresh_var();

                        let mut substed_inner = mem::replace(inner, Box::new(Term::Error)); // TODO: using mem::replace so I can own it, but might not be the most efficient.
                        replace_var(&mut substed_inner, var, &fresh_var);

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

fn replace_var(_term: &mut Term, _original: &Var, _new: &Var) {
    todo!()
}
