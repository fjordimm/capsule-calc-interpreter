use capsule_calc_interpreter::{
    capsule_calc::{
        Environment,
        garbage_collection::collect_garbage,
        interpreting::{is_irreducible, take_step_lazy},
    },
    cli::start_cli,
    compiler::compile,
};

// fn main() -> std::io::Result<()> {
//     let input = std::fs::read_to_string("examples/test2.txt")?;

//     let term = compile(&input);
//     let environment = Environment::new();

//     start_cli(term, environment)
// }

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/test2.txt")?;

    let mut term = compile(&input);
    let mut environment = Environment::new();

    for _ in 0..100 {
        if is_irreducible(&term) {
            break;
        }
        take_step_lazy(&mut term, &mut environment);
    }
    collect_garbage(&term, &mut environment);

    Ok(())
}

// TODO: make step rule for ITE
// TODO: maybe use pairs instead of having a special step thing for constant functions with two args.
// TODO: make fresh vars more readable (maybe have a counter instead of adding a `'`)
// TODO: optimize when to do garbage collection
// TODO: term equivalence tester
// TODO: make a warning for if a user-defined variable is not [_a-zA-Z0-9]
// TODO: test lazy eval with a cycle
