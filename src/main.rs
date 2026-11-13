use capsule_calc_interpreter::{capsule_calc::Environment, cli::start_cli, compiler::compile};

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/test2.txt")?;

    let term = compile(&input);
    let environment = Environment::new();

    start_cli(term, environment)
}

// TODO: make fresh vars more readable (maybe have a counter instead of adding a `'`)
// TODO: make thing for CLI that does n iterations
// TODO: optimize when to do garbage collection
// TODO: term equivalence tester
// TODO: lazy evaluation
