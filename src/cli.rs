use crate::capsule_calc::garbage_collection::collect_garbage;
use crate::capsule_calc::interpreting::{is_irreducible, take_step_lazy};
use crate::capsule_calc::{Environment, Term};
use crate::capsule_calc::{displaying::display_capsule, interpreting::take_step};
use std::io::{self, Write};

pub fn start_cli(mut term: Box<Term>, mut environment: Environment) -> io::Result<()> {
    let stdin = io::stdin();
    let mut iteration = 1;
    let mut doing_lazy = false;

    loop {
        println!("{}", "-".repeat(80));
        println!("Iteration #{}", iteration);
        match is_irreducible(&term) {
            true => println!("(NOT reducible)"),
            false => println!("(reducible)"),
        }
        println!();
        println!("{}", display_capsule(&term, &environment));
        println!();
        print!("Enter command, or `h` for help: ");
        io::stdout().flush()?;

        let mut input = String::new();
        loop {
            input.clear();
            if stdin.read_line(&mut input)? == 0 {
                return Ok(());
            }

            match input.trim() {
                "h" => {
                    println!();
                    println!("Commands:");
                    println!("  Enter  Take a step and collect garbage");
                    println!("  s      Take a step without collecting garbage");
                    println!("  g      Collect garbage");
                    #[rustfmt::skip]
                    println!("  <n>    Take up to n steps, stopping if irreducible, and collect garbage");
                    #[rustfmt::skip]
                    println!("  <n>s   Take up to n steps, stopping if irreducible, without collecting garbage");
                    println!("  l      Toggle lazy evaluation");
                    println!("  h      Display this help message");
                    println!("  q      Quit");
                    println!();
                    print!("Enter command, or `h` for help: ");
                    io::stdout().flush()?;
                }
                "q" => return Ok(()),
                "" => {
                    take_step_lazy_or_not(&mut term, &mut environment, doing_lazy);
                    collect_garbage(&term, &mut environment);
                    iteration += 1;
                    break;
                }
                "s" => {
                    take_step_lazy_or_not(&mut term, &mut environment, doing_lazy);
                    iteration += 1;
                    break;
                }
                "g" => {
                    collect_garbage(&term, &mut environment);
                    break;
                }
                "l" => {
                    doing_lazy = !doing_lazy;
                    println!();
                    println!(
                        "Lazy evaluation is now {}",
                        match doing_lazy {
                            true => "ON",
                            false => "OFF",
                        }
                    );
                    println!();
                    print!("Enter command, or `h` for help: ");
                    io::stdout().flush()?;
                }
                command if command.ends_with('s') => {
                    let steps = command.strip_suffix('s').unwrap_or("");
                    match steps.parse::<usize>() {
                        Ok(steps) => {
                            for _ in 0..steps {
                                if is_irreducible(&term) {
                                    break;
                                }
                                take_step_lazy_or_not(&mut term, &mut environment, doing_lazy);
                                iteration += 1;
                            }
                            break;
                        }
                        _ => {
                            println!();
                            println!("Unknown command.");
                            println!();
                            print!("Enter command, or `h` for help: ");
                            io::stdout().flush()?;
                        }
                    }
                }
                command => match command.parse::<usize>() {
                    Ok(steps) => {
                        for _ in 0..steps {
                            if is_irreducible(&term) {
                                break;
                            }
                            take_step_lazy_or_not(&mut term, &mut environment, doing_lazy);
                            iteration += 1;
                        }
                        collect_garbage(&term, &mut environment);
                        break;
                    }
                    _ => {
                        println!();
                        println!("Unknown command.");
                        println!();
                        print!("Enter command, or `h` for help: ");
                        io::stdout().flush()?;
                    }
                },
            }
        }
    }
}

fn take_step_lazy_or_not(term: &mut Box<Term>, environment: &mut Environment, doing_lazy: bool) {
    if doing_lazy {
        take_step_lazy(term, environment);
    } else {
        take_step(term, environment);
    }
}
