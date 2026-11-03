use capsule_calc_interpreter::{
    capsule_calc::displaying::display_term,
    compiler::{lexing::lex, parsing::parse},
};

// fn main() -> std::io::Result<()> {
//     let term = Box::new(t_application(
//         t_application(t_const(DIV()), t_const(c_int(5))),
//         t_const(c_int(0)),
//     ));
//     let environment = Environment::new();

//     start_cli(term, environment)
// }

fn main() {
    let lexemes = lex("(\\ x -> Unit) (a b c)");
    println!("{:?}", lexemes);
    println!();
    println!("{}", display_term(&parse(&lexemes)));
}

// TODO: test Landin's knot
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection (make the step function takes a parameter that garbage collects)
// TODO: parser
// TODO: equivalence tester
