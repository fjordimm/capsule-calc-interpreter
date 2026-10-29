use capsule_calc_interpreter::capsule_calc::{
    Constant, Environment, Term, displaying::display_capsule, interpreting::take_step,
    some_funcs::incr,
};

fn main() {
    let mut term = Box::new(Term::application(incr(), Term::constant(Constant::int(13))));
    let mut env = Environment::new();

    println!("---------------------------------------------");
    println!("{}", display_capsule(&term, &env));

    take_step(&mut term, &mut env);

    println!("---------------------------------------------");
    println!("{}", display_capsule(&term, &env));
}
