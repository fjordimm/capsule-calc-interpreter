use crate::capsule_calc::{Environment, Term, some_or_panic_r};

pub fn display_capsule(term: &Term, environment: &Environment) -> String {
    let mut display = String::new();

    display.push_str("===Environment===\n");
    display.push_str(&display_environment(environment));
    display.push_str("\n===Term===\n");
    display.push_str(&display_term(term));

    display
}

pub fn display_term(term: &Term) -> String {
    term.tree_display(0, true)
}

/// Renders a value as an indented, one-node-per-line tree.
pub trait TreeDisplayable {
    fn tree_display(&self, indent: i32, is_last: bool) -> String;
}

fn indentation(indent: i32, is_last: bool) -> String {
    let indent = indent.max(0) as usize;

    if indent == 0 {
        return String::new();
    }

    let mut display = "│ ".repeat(indent - 1);
    display.push(if is_last { '└' } else { '├' });
    display.push('─');
    display
}

fn make_tree_string(
    indent: i32,
    is_last: bool,
    label: impl Into<String>,
    children: &[String],
) -> String {
    let mut display = format!("{}{}", indentation(indent, is_last), label.into());

    for child in children {
        display.push('\n');
        display.push_str(child);
    }

    display
}

impl TreeDisplayable for Term {
    fn tree_display(&self, indent: i32, is_last: bool) -> String {
        match self {
            Term::Const(value) => {
                make_tree_string(indent, is_last, format!("Const: {}", value.as_str()), &[])
            }
            Term::Var(variable) => {
                make_tree_string(indent, is_last, format!("Var: {}", variable.as_str()), &[])
            }
            Term::Lambda(parameter, body) => make_tree_string(
                indent,
                is_last,
                format!("Lambda: {}", parameter.as_str()),
                &[some_or_panic_r(body).tree_display(indent + 1, true)],
            ),
            Term::Application(function, argument) => make_tree_string(
                indent,
                is_last,
                "Application",
                &[
                    some_or_panic_r(function).tree_display(indent + 1, false),
                    some_or_panic_r(argument).tree_display(indent + 1, true),
                ],
            ),
            Term::Assignment(variable, value) => make_tree_string(
                indent,
                is_last,
                format!("Assignment: {}", variable.as_str()),
                &[some_or_panic_r(value).tree_display(indent + 1, true)],
            ),
        }
    }
}

pub fn display_environment(env: &Environment) -> String {
    if env.0.is_empty() {
        String::from("(empty)\n")
    } else {
        let mut display = String::new();

        env.0.iter().for_each(|(var, term)| {
            display.push_str(&format!("- {} :=\n", var.as_str()));
            display.push_str(&display_term(term));
            display.push('\n');
        });

        display
    }
}
