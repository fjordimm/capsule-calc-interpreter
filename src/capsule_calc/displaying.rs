use crate::capsule_calc::{some_or_panic_r, Environment, Term};

pub fn display_capsule(term: &Term, environment: &Environment) -> String {
    let mut display = String::new();

    display.push_str("===Environment===\n");
    display.push_str(&display_environment(environment));
    display.push_str("\n===Term===\n");
    display.push_str(&display_term(term));

    display
}

pub fn display_term(term: &Term) -> String {
    term.tree_display(String::new(), true)
}

/// Renders a value as an indented, one-node-per-line tree.
pub trait TreeDisplayable {
    fn tree_display(&self, indent: String, is_last: bool) -> String;
}

fn indentation(indent: String, is_last: bool) -> String {
    if indent.is_empty() {
        return String::new();
    }

    let parent_indent = indent
        .strip_suffix("├─")
        .or_else(|| indent.strip_suffix("└─"))
        .unwrap_or(&indent);

    format!("{}{}", parent_indent, if is_last { "  " } else { "│ " })
}

fn make_tree_string(indent: String, label: impl Into<String>, children: &[String]) -> String {
    let mut display = format!("{}{}", indent, label.into());

    for child in children {
        display.push('\n');
        display.push_str(child);
    }

    display
}

impl TreeDisplayable for Term {
    fn tree_display(&self, indent: String, is_last: bool) -> String {
        let child_indent = indentation(indent.clone(), is_last);

        match self {
            Term::Const(value) => {
                make_tree_string(indent, format!("Const: {}", value.as_str()), &[])
            }
            Term::Var(variable) => {
                make_tree_string(indent, format!("Var: {}", variable.as_str()), &[])
            }
            Term::Lambda(parameter, body) => make_tree_string(
                indent,
                format!("Lambda: {}", parameter.as_str()),
                &[some_or_panic_r(body).tree_display(format!("{}└─", child_indent), true)],
            ),
            Term::Application(function, argument) => make_tree_string(
                indent,
                "Application",
                &[
                    some_or_panic_r(function).tree_display(format!("{}├─", child_indent), false),
                    some_or_panic_r(argument).tree_display(format!("{}└─", child_indent), true),
                ],
            ),
            Term::Assignment(variable, value) => make_tree_string(
                indent,
                format!("Assignment: {}", variable.as_str()),
                &[some_or_panic_r(value).tree_display(format!("{}└─", child_indent), true)],
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

#[cfg(test)]
mod tests {
    use super::display_term;
    use crate::capsule_calc::{t_application, t_var, variable};

    #[test]
    fn displays_nested_branches_with_indentation_prefixes() {
        let term = t_application(
            t_var(variable("f")),
            t_application(t_var(variable("x")), t_var(variable("y"))),
        );

        assert_eq!(
            display_term(&term),
            "Application\n├─Var: f\n└─Application\n  ├─Var: x\n  └─Var: y"
        );
    }
}
