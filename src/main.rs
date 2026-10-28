use capsule_calc_interpreter::capsule_calc::{Constant, Term, Var, displaying::display_term};

fn main() {
    // let a = Term::Lambda(
    //     Var("a".to_string()),
    //     Box::new(Term::Constant(Constant::Int(5))),
    // );

    // let a = Term::lambda(Var::new("x"), Term::constant(Constant::Int(15)));
    let a = Term::if_then_else(
        Term::constant(Constant::Bool(true)),
        Term::application(
            Term::lambda(Var::new("x"), Term::Var(Var::new("x"))),
            Term::constant(Constant::Int(100)),
        ),
        Term::constant(Constant::Int(200)),
    );

    println!("{}", display_term(a));
}
