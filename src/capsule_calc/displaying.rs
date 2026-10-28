use super::{Constant, Term};

pub fn display_term(term: Term) -> String {
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

fn node(indent: i32, is_last: bool, label: impl Into<String>, children: &[String]) -> String {
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
            Term::Constant(value) => node(indent, is_last, constant_label(value), &[]),
            Term::Var(variable) => {
                node(indent, is_last, format!("Var({})", variable.as_str()), &[])
            }
            Term::Lambda(parameter, body) => node(
                indent,
                is_last,
                format!("Lambda({})", parameter.as_str()),
                &[body.tree_display(indent + 1, true)],
            ),
            Term::Application(function, argument) => node(
                indent,
                is_last,
                "Application",
                &[
                    function.tree_display(indent + 1, false),
                    argument.tree_display(indent + 1, true),
                ],
            ),
            Term::Assignment(variable, value) => node(
                indent,
                is_last,
                format!("Assignment({})", variable.as_str()),
                &[value.tree_display(indent + 1, true)],
            ),
            Term::Composition(first, second) => node(
                indent,
                is_last,
                "Composition",
                &[
                    first.tree_display(indent + 1, false),
                    second.tree_display(indent + 1, true),
                ],
            ),
            Term::IfThenElse(condition, then_branch, else_branch) => node(
                indent,
                is_last,
                "IfThenElse",
                &[
                    condition.tree_display(indent + 1, false),
                    then_branch.tree_display(indent + 1, false),
                    else_branch.tree_display(indent + 1, true),
                ],
            ),
            Term::RepeatUntil(body, condition) => node(
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

fn constant_label(value: &Constant) -> String {
    match value {
        Constant::None => "Constant(None)".to_string(),
        Constant::Bool(value) => format!("Constant(Bool({value}))"),
        Constant::Int(value) => format!("Constant(Int({value}))"),
    }
}
