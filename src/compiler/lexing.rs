#[derive(Debug, PartialEq, Eq)]
pub enum Lexeme {
    LParen,
    RParen,
    Backslash,
    Arrow,
    Semicolon,
    Assignment,
    Equals,
    NotEquals,
    And,
    Or,
    Leq,
    Geq,
    Lt,
    Gt,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Slash,
    If,
    Then,
    Else,
    While,
    Do,
    Let,
    Be,
    In,
    Letrec,
    LitError,
    LitUnit,
    LitTrue,
    LitFalse,
    LitNum(u32),
    Identifier(String),
}

// Lexer written by Copilot.

pub fn lex(input: &str) -> Vec<Lexeme> {
    let chars: Vec<char> = input.chars().collect();
    let mut lexemes = Vec::new();
    let mut position = 0;

    while position < chars.len() {
        let current = chars[position];

        if current.is_whitespace() {
            position += 1;
            continue;
        }

        if current.is_ascii_digit() {
            let start = position;
            while position < chars.len() && chars[position].is_ascii_digit() {
                position += 1;
            }

            let number: String = chars[start..position].iter().collect();
            lexemes.push(Lexeme::LitNum(
                number
                    .parse()
                    .expect("numeric literal is out of range for u32"),
            ));
            continue;
        }

        if is_identifier_start(current) {
            let start = position;
            position += 1;
            while position < chars.len() && is_identifier_continue(chars[position]) {
                position += 1;
            }

            let identifier: String = chars[start..position].iter().collect();
            lexemes.push(match identifier.as_str() {
                "if" => Lexeme::If,
                "then" => Lexeme::Then,
                "else" => Lexeme::Else,
                "while" => Lexeme::While,
                "do" => Lexeme::Do,
                "let" => Lexeme::Let,
                "be" => Lexeme::Be,
                "in" => Lexeme::In,
                "letrec" => Lexeme::Letrec,
                "Error" => Lexeme::LitError,
                "Unit" => Lexeme::LitUnit,
                "True" => Lexeme::LitTrue,
                "False" => Lexeme::LitFalse,
                _ => Lexeme::Identifier(identifier),
            });
            continue;
        }

        let next = chars.get(position + 1).copied();
        let two_character_lexeme = match (current, next) {
            ('-', Some('>')) => Some(Lexeme::Arrow),
            (':', Some('=')) => Some(Lexeme::Assignment),
            ('=', Some('=')) => Some(Lexeme::Equals),
            ('!', Some('=')) => Some(Lexeme::NotEquals),
            ('&', Some('&')) => Some(Lexeme::And),
            ('|', Some('|')) => Some(Lexeme::Or),
            ('<', Some('=')) => Some(Lexeme::Leq),
            ('>', Some('=')) => Some(Lexeme::Geq),
            _ => None,
        };
        if let Some(lexeme) = two_character_lexeme {
            lexemes.push(lexeme);
            position += 2;
            continue;
        }

        let lexeme = match current {
            '(' => Lexeme::LParen,
            ')' => Lexeme::RParen,
            '\\' => Lexeme::Backslash,
            ';' => Lexeme::Semicolon,
            '<' => Lexeme::Lt,
            '>' => Lexeme::Gt,
            '!' => Lexeme::Bang,
            '+' => Lexeme::Plus,
            '-' => Lexeme::Minus,
            '*' => Lexeme::Asterisk,
            '/' => Lexeme::Slash,
            _ => panic!("Unexpected character {current:?} at position {position}"),
        };
        lexemes.push(lexeme);
        position += 1;
    }

    lexemes
}

// Written by Copilot.
fn is_identifier_start(character: char) -> bool {
    character == '_' || character.is_ascii_alphabetic()
}

// Written by Copilot.
fn is_identifier_continue(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}

// Written by Copilot.
#[cfg(test)]
mod tests {
    use super::{Lexeme, lex};

    #[test]
    fn lexes_operators_including_multi_character_operators() {
        assert_eq!(
            lex("( ) \\ -> ; := == != && || < > <= >= ! + - * /"),
            vec![
                Lexeme::LParen,
                Lexeme::RParen,
                Lexeme::Backslash,
                Lexeme::Arrow,
                Lexeme::Semicolon,
                Lexeme::Assignment,
                Lexeme::Equals,
                Lexeme::NotEquals,
                Lexeme::And,
                Lexeme::Or,
                Lexeme::Lt,
                Lexeme::Gt,
                Lexeme::Leq,
                Lexeme::Geq,
                Lexeme::Bang,
                Lexeme::Plus,
                Lexeme::Minus,
                Lexeme::Asterisk,
                Lexeme::Slash,
            ]
        );
    }

    #[test]
    fn ignores_whitespace_between_tokens_and_separates_identifiers() {
        assert_eq!(
            lex("  alpha_1\tbeta\n"),
            vec![
                Lexeme::Identifier("alpha_1".to_owned()),
                Lexeme::Identifier("beta".to_owned()),
            ]
        );
    }

    #[test]
    fn recognizes_keywords_only_as_complete_identifiers() {
        assert_eq!(
            lex("if then else while do let be in letrec Error Unit True False ifx"),
            vec![
                Lexeme::If,
                Lexeme::Then,
                Lexeme::Else,
                Lexeme::While,
                Lexeme::Do,
                Lexeme::Let,
                Lexeme::Be,
                Lexeme::In,
                Lexeme::Letrec,
                Lexeme::LitError,
                Lexeme::LitUnit,
                Lexeme::LitTrue,
                Lexeme::LitFalse,
                Lexeme::Identifier("ifx".to_owned()),
            ]
        );
    }

    #[test]
    fn lexes_numeric_literals_as_u32() {
        assert_eq!(
            lex("0 999999999"),
            vec![Lexeme::LitNum(0), Lexeme::LitNum(999999999)]
        );
    }

    #[test]
    fn operators_do_not_join_across_whitespace() {
        assert_eq!(lex("- >"), vec![Lexeme::Minus, Lexeme::Gt]);
        assert!(std::panic::catch_unwind(|| lex("& &")).is_err());
        assert!(std::panic::catch_unwind(|| lex("| |")).is_err());
    }
}
