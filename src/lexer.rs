use regex::Regex;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Token {
    Int { value: i32 },
    OpenBracket,
    CloseBracket,
    Multiplication,
    Division,
    Addition,
    Subtraction,
    UnknownToken,
}

fn is_int(str: &str) -> bool {
    let int_regex = Regex::new(r"^\d*$").unwrap();
    int_regex.is_match(str)
}

fn to_int(known_int_str: &str) -> Token {
    Token::Int { value: known_int_str.parse::<i32>().unwrap() }
}

pub fn lex(str: &str) -> Vec<Token> {
    let split_on_whitespace = str.split(" ").into_iter();
    split_on_whitespace.map(|char| {
        match char {
            "(" => { Token::OpenBracket }
            ")" => { Token::CloseBracket }
            "*" => { Token::Multiplication }
            "+" => { Token::Addition }
            "/" => { Token::Division }
            "-" => { Token::Subtraction }
            number if is_int(number) => { to_int(number) }
            _ => { Token::UnknownToken }
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTEGER_TOKEN_VALUE_2: Token = Token::Int { value: 2 };

    fn sum_tokens(i1: i32, i2: i32, operator: Token) -> Vec<Token> {
        vec![Token::OpenBracket, operator, Token::Int {value: i1}, Token::Int {value: i2}, Token::CloseBracket]
    }

    #[test]
    fn should_parse_an_integer() {
        assert_eq!(lex("2"), vec![INTEGER_TOKEN_VALUE_2])
    }

    #[test]
    fn should_parse_an_integer_in_brackets() {
        assert_eq!(lex("( 2 )"), vec![Token::OpenBracket, INTEGER_TOKEN_VALUE_2, Token::CloseBracket])
    }


    #[test]
    fn should_parse_multiplication() {
        let expected_tokens = sum_tokens(2, 2, Token::Multiplication);
        assert_eq!(lex("( * 2 2 )"), expected_tokens)
    }

    #[test]
    fn should_parse_addition() {
        let expected_tokens = sum_tokens(2, 2, Token::Addition);
        assert_eq!(lex("( + 2 2 )"), expected_tokens)
    }

    #[test]
    fn should_parse_division() {
        let expected_tokens = sum_tokens(2, 2, Token::Division);
        assert_eq!(lex("( / 2 2 )"), expected_tokens)
    }


    #[test]
    fn should_parse_subtraction() {
        let expected_tokens = sum_tokens(2, 2, Token::Subtraction);
        assert_eq!(lex("( - 2 2 )"), expected_tokens)
    }

    #[test]
    fn should_return_an_unknown_token_for_a_string_for_now() {
        assert_eq!(lex("hello"), vec![Token::UnknownToken])
    }
}