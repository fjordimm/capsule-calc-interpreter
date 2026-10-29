use crate::capsule_calc::{Environment, Term};

pub fn take_step(term: &mut Box<Term>, environment: &mut Environment) {
    match term.as_mut() {
        Term::Error => {}
        Term::Constant(_) => {}
        Term::Var(var) => {
            match environment.lookup(var) {
                None => **term = Term::error(),
                Some(value) => *term = value.clone(), // TODO: this is not lazy evaluation.
            };
        }
        Term::Lambda(_, _) => {}
        Term::Application(t1, t2) => {
            if !is_irreducible(&t1) {
                take_step(t1, environment);
            } else if !is_irreducible(&t2) {
                take_step(t2, environment);
            } else {
                todo!()
            }
        }
        _ => todo!(),
    }
}

// fn apply(term: &mut )

pub fn is_irreducible(term: &Term) -> bool {
    match term {
        Term::Error => true,
        Term::Constant(_) => true,
        Term::Lambda(_, _) => true,
        _ => false,
    }
}
