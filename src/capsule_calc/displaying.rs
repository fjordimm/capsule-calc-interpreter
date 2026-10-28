use super::{Constant, Term};

pub fn display_term(term: Term) -> String {
    term.tree_display(0)
}

/// Renders a value as an indented, one-node-per-line tree.
trait TreeDisplayable {
    fn tree_display(&self, indent: i32) -> String;
}

fn indentation(indent: i32) -> String {
    "  ".repeat(indent.max(0) as usize)
}

fn node(indent: i32, label: impl Into<String>, children: &[String]) -> String {
    let mut display = format!("{}{}", indentation(indent), label.into());

    for child in children {
        display.push('\n');
        display.push_str(child);
    }

    display
}

impl TreeDisplayable for Term {
    fn tree_display(&self, indent: i32) -> String {
        match self {
            Term::Constant(value) => node(indent, constant_label(value), &[]),
            Term::Var(variable) => node(indent, format!("Var({})", variable.as_str()), &[]),
            Term::Lambda(parameter, body) => node(
                indent,
                format!("Lambda({})", parameter.as_str()),
                &[body.tree_display(indent + 1)],
            ),
            Term::Application(function, argument) => node(
                indent,
                "Application",
                &[
                    function.tree_display(indent + 1),
                    argument.tree_display(indent + 1),
                ],
            ),
            Term::Assignment(variable, value) => node(
                indent,
                format!("Assignment({})", variable.as_str()),
                &[value.tree_display(indent + 1)],
            ),
            Term::Composition(first, second) => node(
                indent,
                "Composition",
                &[
                    first.tree_display(indent + 1),
                    second.tree_display(indent + 1),
                ],
            ),
            Term::IfThenElse(condition, then_branch, else_branch) => node(
                indent,
                "IfThenElse",
                &[
                    condition.tree_display(indent + 1),
                    then_branch.tree_display(indent + 1),
                    else_branch.tree_display(indent + 1),
                ],
            ),
            Term::RepeatUntil(body, condition) => node(
                indent,
                "RepeatUntil",
                &[
                    body.tree_display(indent + 1),
                    condition.tree_display(indent + 1),
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
