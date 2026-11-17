use capsule_calc_interpreter::{
    capsule_calc::{Environment, LazyEnvironment},
    cli::{start_cli, start_cli_lazy},
    compiler::compile,
};

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/test2.txt")?;

    if false {
        let term = compile(&input);
        let environment = Environment::new();

        start_cli(term, environment)
    } else {
        let term = compile(&input);
        let environment = LazyEnvironment::new();

        start_cli_lazy(term, environment)
    }
}

// TODO: make step rule for ITE
// TODO: maybe use pairs instead of having a special step thing for constant functions with two args.
// TODO: make fresh vars more readable (maybe have a counter instead of adding a `'`)
// TODO: optimize when to do garbage collection
// TODO: term equivalence tester
// TODO: make a warning for if a user-defined variable is not [_a-zA-Z0-9]
// TODO: test lazy eval with a cycle
