use capsule_calc_interpreter::{
    capsule_calc::Environment,
    cli::start_cli,
    compiler::{lexing::lex, parsing::parse},
};

fn main() -> std::io::Result<()> {
    let input = "letrec f be \\x -> (
        if x == 0 then 1
        else if x == 1 then 1
        else f (x - 1) + f (x - 2)
    ) in f 5";

    let term = parse(&lex(input));
    let environment = Environment::new();

    start_cli(term, environment)
}

// TODO: make fresh vars more readable
// TODO: test Landin's knot
//  - right now it works, but using a recursive if-then-else won't terminate.
// TODO: make thing for CLI that does n iterations
// TODO: garbage collection (make the step function takes a parameter that garbage collects)
// TODO: term equivalence tester
// TODO: lazy evaluation and stuff
