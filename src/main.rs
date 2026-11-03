use capsule_calc_interpreter::{
    capsule_calc::Environment,
    cli::start_cli,
    compiler::{lexing::lex, parsing::parse},
};

fn main() -> std::io::Result<()> {
    let input = "a := 1 + 2";

    let term = parse(&lex(input));
    let environment = Environment::new();

    start_cli(term, environment)
}

// TODO: test Landin's knot
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection (make the step function takes a parameter that garbage collects)
// TODO: parser
// TODO: equivalence tester
