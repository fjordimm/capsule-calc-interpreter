use crate::capsule_calc::{Environment, Term};

pub fn display_capsule(term: &Term, environment: &Environment) -> String {
    let mut display = String::new();

    display.push_str("===Term===\n");
    display.push_str(&display_term(term));
    display.push_str("\n===Environment===\n");
    display.push_str(&display_environment(environment));

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
            Self::Error => make_tree_string(indent, is_last, "Error", &[]),
            Term::Const(value) => make_tree_string(
                indent,
                is_last,
                format!("Constant: {}", value.as_str()),
                &[],
            ),
            Term::Var(variable) => {
                make_tree_string(indent, is_last, format!("Var: {}", variable.as_str()), &[])
            }
            Term::Lambda(parameter, body) => make_tree_string(
                indent,
                is_last,
                format!("Lambda: {}", parameter.as_str()),
                &[body.tree_display(indent + 1, true)],
            ),
            Term::Application(function, argument) => make_tree_string(
                indent,
                is_last,
                "Application",
                &[
                    function.tree_display(indent + 1, false),
                    argument.tree_display(indent + 1, true),
                ],
            ),
            Term::Assignment(variable, value) => make_tree_string(
                indent,
                is_last,
                format!("Assignment: {}", variable.as_str()),
                &[value.tree_display(indent + 1, true)],
            ),
            Term::Composition(first, second) => make_tree_string(
                indent,
                is_last,
                "Composition",
                &[
                    first.tree_display(indent + 1, false),
                    second.tree_display(indent + 1, true),
                ],
            ),
            Term::IfThenElse(condition, then_branch, else_branch) => make_tree_string(
                indent,
                is_last,
                "IfThenElse",
                &[
                    condition.tree_display(indent + 1, false),
                    then_branch.tree_display(indent + 1, false),
                    else_branch.tree_display(indent + 1, true),
                ],
            ),
            Term::RepeatUntil(body, condition) => make_tree_string(
                indent,
                is_last,
                "RepeatUntil",
                &[
                    body.tree_display(indent + 1, false),
                    condition.tree_display(indent + 1, true),
                ],
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
