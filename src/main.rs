use capsule_calc_interpreter::capsule_calc::Var;
use capsule_calc_interpreter::capsule_calc::{Constant, Environment, Term, some_funcs::incr};
use capsule_calc_interpreter::cli::start_cli;

fn main() -> std::io::Result<()> {
    let mut term = Box::new(Term::application(incr(), Term::constant(Constant::int(13))));
    let mut env = Environment::new().with(Var::var("bob"), Term::constant(Constant::bool(false)));

    start_cli(&mut term, &mut env)
}
