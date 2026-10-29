use capsule_calc_interpreter::capsule_calc::{
    CapsuleState, Constant, Environment, Term, Var,
    displaying::{display_capsule, display_term},
    interpreting::take_step,
};

fn main() {
    let mut cap = CapsuleState::new(
        Term::if_then_else(
            Term::constant(Constant::Bool(true)),
            Term::application(
                Term::lambda(Var::var("x"), Term::Var(Var::var("x"))),
                Term::constant(Constant::Int(100)),
            ),
            Term::constant(Constant::Int(200)),
        ),
        Environment::new()
            .with(Var::var("thing"), Term::constant(Constant::Int(21)))
            .with(Var::var("c"), Term::constant(Constant::Bool(false))),
    );

    println!("{}", display_capsule(&cap));

    take_step(&mut cap);

    println!("{}", display_capsule(&cap));
}
