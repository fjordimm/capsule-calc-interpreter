use crate::capsule_calc::interpreting::is_irreducible;
use crate::capsule_calc::{Environment, Term};
use crate::capsule_calc::{displaying::display_capsule, interpreting::take_step};
use std::io::{self, Write};

pub fn start_cli(term: &mut Box<Term>, environment: &mut Environment) -> io::Result<()> {
    let stdin = io::stdin();
    let mut iteration = 1;

    loop {
        println!("{}", "-".repeat(80));
        println!("Iteration #{}", iteration);
        match is_irreducible(term) {
            true => println!("(NOT reducible)"),
            false => println!("(reducible)"),
        }
        println!();
        println!("{}", display_capsule(term, environment));
        print!("Enter to take step. Q to end: ");
        io::stdout().flush()?;

        let mut input = String::new();
        if stdin.read_line(&mut input)? == 0 || input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        take_step(term, environment);
        iteration += 1;
    }

    Ok(())
}
