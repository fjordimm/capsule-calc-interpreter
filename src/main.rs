use capsule_calc_interpreter::capsule_calc::some_funcs::INCR;
use capsule_calc_interpreter::capsule_calc::{Constant, Environment, Term, Var};
use capsule_calc_interpreter::cli::start_cli;

fn main() -> std::io::Result<()> {
    let mut term = Box::new(Term::application(
        Term::lambda(
            Var::var("x"),
            Term::application(INCR(), Term::var(Var::var("x"))),
        ),
        Term::constant(Constant::int(100)),
    ));
    let mut env = Environment::new();

    start_cli(&mut term, &mut env)
}
