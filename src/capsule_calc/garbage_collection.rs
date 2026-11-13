use std::collections::HashSet;

use crate::capsule_calc::{Environment, Term, Variable, some_or_panic_r};

/// Uses a tracing mark-and-sweep method.
pub fn collect_garbage(term: &Term, environment: &mut Environment) {
    let mut vars_in_question = free_vars(term);
    let mut visited_vars = HashSet::<&Variable>::new();

    'main_loop: loop {

    }

    
}

fn free_vars(term: &Term) -> HashSet<&Variable> {
    match term {
        Term::Const(_) => HashSet::new(),
        Term::Var(var) => HashSet::from([var]),
        Term::Lambda(param, body) => {
            let mut vars = free_vars(some_or_panic_r(body));
            vars.remove(param);
            vars
        }
        Term::Application(t1, t2) => {
            let mut vars = free_vars(some_or_panic_r(t1));
            vars.extend(free_vars(some_or_panic_r(t2)));
            vars
        }
        Term::Assignment(_, value) => free_vars(some_or_panic_r(value)),
    }
}
