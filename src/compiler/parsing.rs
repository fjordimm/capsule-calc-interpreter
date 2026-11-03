use crate::{
    capsule_calc::collection_of_const_funcs::{
        ADD, AND, DIV, EQ, GEQ, GT, LEQ, LT, MUL, NEQ, NOT, OR, SUB,
    },
    capsule_calc::{
        c_bool, c_error, c_int, c_unit, t_application, t_assignment, t_const, t_lambda, t_var,
        tt_if_then_else, tt_let_be_in, tt_letrec_be_in, tt_while_do, variable, Constant, Term,
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
            Some(Lexeme::If) => self.parse_if_then_else(),
            Some(Lexeme::While) => self.parse_while_do(),
            Some(Lexeme::Let) => self.parse_let_be_in(false),
            Some(Lexeme::Letrec) => self.parse_let_be_in(true),
            _ => self.parse_assignment(),
        }
    }

    fn parse_if_then_else(&mut self) -> Box<Term> {
        self.position += 1;
        let condition = self.parse_term();
        self.expect_keyword(Lexeme::Then, "then");
        let then_case = self.parse_term();
        self.expect_keyword(Lexeme::Else, "else");
        let else_case = self.parse_term();

        Box::new(tt_if_then_else(*condition, *then_case, *else_case))
    }

    fn parse_while_do(&mut self) -> Box<Term> {
        self.position += 1;
        let condition = self.parse_term();
        self.expect_keyword(Lexeme::Do, "do");
        let body = self.parse_term();

        Box::new(tt_while_do(*condition, *body))
    }

    fn parse_let_be_in(&mut self, recursive: bool) -> Box<Term> {
        self.expect_keyword(
            if recursive {
                Lexeme::Letrec
            } else {
                Lexeme::Let
            },
            if recursive { "letrec" } else { "let" },
        );

        let bound_variable = match self.next() {
            Some(Lexeme::Identifier(name)) => variable(name.clone()),
            token => panic!("Expected identifier after let, found {token:?}"),
        };
        self.expect_keyword(Lexeme::Be, "be");
        let value = self.parse_term();
        self.expect_keyword(Lexeme::In, "in");
        let body = self.parse_term();

        if recursive {
            Box::new(tt_letrec_be_in(bound_variable, *value, *body))
        } else {
            Box::new(tt_let_be_in(bound_variable, *value, *body))
        }
    }

    fn expect_keyword(&mut self, expected: Lexeme, name: &str) {
        match self.next() {
            Some(token) if token == &expected => {}
            token => panic!("Expected {name}, found {token:?}"),
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

    fn parse_assignment(&mut self) -> Box<Term> {
        let left = self.parse_or();

        if matches!(self.peek(), Some(Lexeme::Assignment)) {
            self.position += 1;
            let variable = match *left {
                Term::Var(variable) => variable,
                _ => panic!("the left side of an assignment must be an identifier"),
            };
            let right = self.parse_assignment();
            Box::new(t_assignment(variable, *right))
        } else {
            left
        }
    }

    fn parse_or(&mut self) -> Box<Term> {
        self.parse_left_associative(
            |parser| parser.parse_and(),
            |token| matches!(token, Lexeme::Or).then(OR),
        )
    }

    fn parse_and(&mut self) -> Box<Term> {
        self.parse_left_associative(
            |parser| parser.parse_comparison(),
            |token| matches!(token, Lexeme::And).then(AND),
        )
    }

    fn parse_comparison(&mut self) -> Box<Term> {
        self.parse_left_associative(
            |parser| parser.parse_additive(),
            |token| match token {
                Lexeme::Equals => Some(EQ()),
                Lexeme::NotEquals => Some(NEQ()),
                Lexeme::Lt => Some(LT()),
                Lexeme::Gt => Some(GT()),
                Lexeme::Leq => Some(LEQ()),
                Lexeme::Geq => Some(GEQ()),
                _ => None,
            },
        )
    }

    fn parse_additive(&mut self) -> Box<Term> {
        self.parse_left_associative(
            |parser| parser.parse_multiplicative(),
            |token| match token {
                Lexeme::Plus => Some(ADD()),
                Lexeme::Minus => Some(SUB()),
                _ => None,
            },
        )
    }

    fn parse_multiplicative(&mut self) -> Box<Term> {
        self.parse_left_associative(
            |parser| parser.parse_application(),
            |token| match token {
                Lexeme::Asterisk => Some(MUL()),
                Lexeme::Slash => Some(DIV()),
                _ => None,
            },
        )
    }

    fn parse_left_associative(
        &mut self,
        parse_operand: impl Fn(&mut Self) -> Box<Term>,
        get_operator: impl Fn(&Lexeme) -> Option<Constant>,
    ) -> Box<Term> {
        let mut term = parse_operand(self);

        while let Some(function) = self.peek().and_then(|token| get_operator(token)) {
            self.position += 1;
            let right = parse_operand(self);
            term = Box::new(apply_binary_operator(*term, *right, function));
        }

        term
    }

    fn parse_application(&mut self) -> Box<Term> {
        let mut term = self.parse_prefix();

        while self.next_token_starts_application_operand() {
            let argument = self.parse_prefix();
            term = Box::new(t_application(*term, *argument));
        }

        term
    }

    fn parse_prefix(&mut self) -> Box<Term> {
        if matches!(self.peek(), Some(Lexeme::Bang)) {
            self.position += 1;
            return Box::new(t_application(t_const(NOT()), *self.parse_prefix()));
        }

        self.parse_atom()
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

    fn next_token_starts_application_operand(&self) -> bool {
        matches!(
            self.peek(),
            Some(
                Lexeme::Bang
                    | Lexeme::LParen
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

fn apply_binary_operator(left: Term, right: Term, function: Constant) -> Term {
    t_application(t_application(t_const(function), left), right)
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
        let result = std::panic::catch_unwind(|| parse(&lex("x ; y")));
        assert!(result.is_err());
    }

    fn binary_operator_name(term: &Term) -> &str {
        let Term::Application(Some(partial), Some(_)) = term else {
            panic!("expected an outer operator application");
        };
        let Term::Application(Some(function), Some(_)) = partial.as_ref() else {
            panic!("expected a partially applied binary operator");
        };
        let Term::Const(crate::capsule_calc::Constant::Func { name, .. }) = function.as_ref()
        else {
            panic!("expected the operator function");
        };
        name
    }

    #[test]
    fn parses_every_binary_operator_as_a_function_application() {
        for (source, function_name) in [
            ("1 + 2", "ADD"),
            ("1 - 2", "SUB"),
            ("1 * 2", "MUL"),
            ("1 / 2", "DIV"),
            ("1 == 2", "EQ"),
            ("1 != 2", "NEQ"),
            ("1 < 2", "LT"),
            ("1 > 2", "GT"),
            ("1 <= 2", "LEQ"),
            ("1 >= 2", "GEQ"),
            ("True && False", "AND"),
            ("True || False", "OR"),
        ] {
            assert_eq!(binary_operator_name(&parse(&lex(source))), function_name);
        }
    }

    #[test]
    fn parses_not_and_assignment() {
        let not_term = parse(&lex("!True"));
        let Term::Application(Some(function), Some(_)) = not_term.as_ref() else {
            panic!("expected NOT application");
        };
        assert!(matches!(
            function.as_ref(),
            Term::Const(crate::capsule_calc::Constant::Func { name, .. }) if name == "NOT"
        ));

        assert!(matches!(*parse(&lex("x := 1")), Term::Assignment(_, _)));
    }

    #[test]
    fn arithmetic_operators_have_conventional_precedence() {
        let term = parse(&lex("1 + 2 * 3"));
        assert_eq!(binary_operator_name(&term), "ADD");

        let Term::Application(Some(_partial), Some(right)) = term.as_ref() else {
            panic!("expected ADD application");
        };
        assert_eq!(binary_operator_name(right), "MUL");
    }

    fn contains_function(term: &Term, expected_name: &str) -> bool {
        match term {
            Term::Const(crate::capsule_calc::Constant::Func { name, .. }) => name == expected_name,
            Term::Application(Some(function), Some(argument)) => {
                contains_function(function, expected_name)
                    || contains_function(argument, expected_name)
            }
            Term::Lambda(_, Some(body)) | Term::Assignment(_, Some(body)) => {
                contains_function(body, expected_name)
            }
            _ => false,
        }
    }

    fn contains_assignment(term: &Term) -> bool {
        match term {
            Term::Assignment(_, _) => true,
            Term::Application(Some(function), Some(argument)) => {
                contains_assignment(function) || contains_assignment(argument)
            }
            Term::Lambda(_, Some(body)) => contains_assignment(body),
            _ => false,
        }
    }

    #[test]
    fn parses_if_then_else_and_while_do() {
        let conditional = parse(&lex("if True then 1 else 2"));
        assert!(contains_function(&conditional, "ITE"));

        let loop_term = parse(&lex("while False do Unit"));
        assert!(contains_function(&loop_term, "ITE"));
        assert!(contains_assignment(&loop_term));
    }

    #[test]
    fn parses_let_and_letrec_with_nested_terms() {
        let let_term = parse(&lex("let x be if True then 1 else 2 in x"));
        assert!(contains_function(&let_term, "ITE"));
        assert!(matches!(
            let_term.as_ref(),
            Term::Application(Some(binding), Some(_))
                if matches!(binding.as_ref(), Term::Lambda(_, _))
        ));

        let letrec_term = parse(&lex("letrec f be \\x -> x in f"));
        assert!(contains_assignment(&letrec_term));
        assert!(matches!(
            letrec_term.as_ref(),
            Term::Application(Some(binding), Some(_))
                if matches!(binding.as_ref(), Term::Lambda(_, _))
        ));
    }
}
