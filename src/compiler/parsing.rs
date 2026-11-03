use crate::{
    capsule_calc::{
        Term, c_bool, c_error, c_int, c_unit, t_application, t_const, t_lambda, t_var, variable,
    },
    compiler::lexing::Lexeme,
};

// Parser written by Copilot.

pub fn parse(input: &[Lexeme]) -> Box<Term> {
    let mut parser = Parser { input, position: 0 };
    let term = parser.parse_term();

    if parser.position != input.len() {
        panic!("Unexpected token after term: {:?}", input[parser.position]);
    }

    term
}

struct Parser<'a> {
    input: &'a [Lexeme],
    position: usize,
}

impl Parser<'_> {
    fn parse_term(&mut self) -> Box<Term> {
        match self.peek() {
            Some(Lexeme::Backslash) => self.parse_lambda(),
            _ => self.parse_application(),
        }
    }

    fn parse_lambda(&mut self) -> Box<Term> {
        self.position += 1;
        let parameter = match self.next() {
            Some(Lexeme::Identifier(name)) => variable(name.clone()),
            token => panic!("Expected lambda parameter, found {token:?}"),
        };

        match self.next() {
            Some(Lexeme::Arrow) => {}
            token => panic!("Expected arrow after lambda parameter, found {token:?}"),
        }

        Box::new(t_lambda(parameter, *self.parse_term()))
    }

    fn parse_application(&mut self) -> Box<Term> {
        let mut term = self.parse_atom();

        while self.next_token_starts_atom() {
            let argument = self.parse_atom();
            term = Box::new(t_application(*term, *argument));
        }

        term
    }

    fn parse_atom(&mut self) -> Box<Term> {
        let token = self.next();
        match token {
            Some(Lexeme::LParen) => {
                let term = self.parse_term();
                match self.next() {
                    Some(Lexeme::RParen) => term,
                    token => panic!("Expected closing parenthesis, found {token:?}"),
                }
            }
            Some(Lexeme::LitError) => Box::new(t_const(c_error())),
            Some(Lexeme::LitUnit) => Box::new(t_const(c_unit())),
            Some(Lexeme::LitTrue) => Box::new(t_const(c_bool(true))),
            Some(Lexeme::LitFalse) => Box::new(t_const(c_bool(false))),
            Some(Lexeme::LitNum(value)) => Box::new(t_const(c_int(*value))),
            Some(Lexeme::Identifier(name)) => Box::new(t_var(variable(name.clone()))),
            token => panic!("Expected a term, found {token:?}"),
        }
    }

    fn next_token_starts_atom(&self) -> bool {
        matches!(
            self.peek(),
            Some(
                Lexeme::LParen
                    | Lexeme::LitError
                    | Lexeme::LitUnit
                    | Lexeme::LitTrue
                    | Lexeme::LitFalse
                    | Lexeme::LitNum(_)
                    | Lexeme::Identifier(_)
            )
        )
    }

    fn peek(&self) -> Option<&Lexeme> {
        self.input.get(self.position)
    }

    fn next(&mut self) -> Option<&Lexeme> {
        let token = self.input.get(self.position);
        if token.is_some() {
            self.position += 1;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::{capsule_calc::Term, compiler::lexing::lex};

    #[test]
    fn parses_constants_and_variables() {
        let cases = [
            ("Error", "Error"),
            ("Unit", "Unit"),
            ("True", "Bool(true)"),
            ("False", "Bool(false)"),
            ("42", "Int(42)"),
        ];

        for (source, expected) in cases {
            match *parse(&lex(source)) {
                Term::Const(value) => assert_eq!(value.as_str(), expected),
                _ => panic!("Expected a constant for {source}"),
            }
        }

        match *parse(&lex("some_var")) {
            Term::Var(variable) => assert_eq!(variable.as_str(), "some_var"),
            _ => panic!("Expected a variable"),
        }
    }

    #[test]
    fn parses_lambda_with_application_body() {
        let parsed = parse(&lex("\\x -> x y"));
        match *parsed {
            Term::Lambda(parameter, Some(body)) => {
                assert_eq!(parameter.as_str(), "x");
                match *body {
                    Term::Application(Some(function), Some(argument)) => {
                        assert!(matches!(*function, Term::Var(_)));
                        match *argument {
                            Term::Var(variable) => assert_eq!(variable.as_str(), "y"),
                            _ => panic!("Expected the argument to be a variable"),
                        }
                    }
                    _ => panic!("Expected an application as the lambda body"),
                }
            }
            _ => panic!("Expected a lambda"),
        }
    }

    #[test]
    fn parses_parenthesized_lambda_application_left_associatively() {
        let parsed = parse(&lex("(\\x -> x) y z"));
        match *parsed {
            Term::Application(Some(function), Some(argument)) => {
                assert!(matches!(*argument, Term::Var(_)));
                assert!(matches!(*function, Term::Application(_, _)));
            }
            _ => panic!("Expected a left-associative application"),
        }
    }

    #[test]
    fn rejects_unsupported_operators() {
        let result = std::panic::catch_unwind(|| parse(&lex("x + y")));
        assert!(result.is_err());
    }
}
