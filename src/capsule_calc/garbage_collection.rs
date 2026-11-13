use std::collections::HashSet;

use crate::capsule_calc::{Environment, Term, Variable, some_or_panic_r};

/// Uses a tracing mark-and-sweep method.
pub fn collect_garbage(term: &Term, environment: &mut Environment) {
    let mut vars_in_question = free_vars(term)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let mut visited_vars = HashSet::<Variable>::new();

    while let Some(var) = vars_in_question.pop() {
        if !visited_vars.insert(var.clone()) {
            continue;
        }

        if let Some(value) = environment.lookup(&var) {
            vars_in_question.extend(free_vars(value).into_iter().cloned());
        }
    }

    environment.0.retain(|var, _| visited_vars.contains(var));
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

#[cfg(test)]
mod tests {
    use super::collect_garbage;
    use crate::capsule_calc::{Environment, Term, c_unit, t_var, variable};

    #[test]
    fn keeps_reachable_bindings_and_collects_unreachable_cycles() {
        let a = variable("a");
        let b = variable("b");
        let x = variable("x");
        let y = variable("y");
        let mut environment = Environment::new()
            .with(a.clone(), t_var(b.clone()))
            .with(b.clone(), Term::Const(c_unit()))
            .with(x.clone(), t_var(y.clone()))
            .with(y.clone(), t_var(x.clone()));

        collect_garbage(&t_var(a.clone()), &mut environment);

        assert!(environment.lookup(&a).is_some());
        assert!(environment.lookup(&b).is_some());
        assert!(environment.lookup(&x).is_none());
        assert!(environment.lookup(&y).is_none());
    }
}
