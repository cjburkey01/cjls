use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Token {
    name: String,
    state_map: Box<[HashMap<char, u32>]>,
}

impl Token {
    fn new(name: &str, state_map: Box<[HashMap<char, u32>]>) -> Self {
        Self {
            name: String::from(name),
            state_map,
        }
    }
}

#[derive(Debug, Clone)]
struct Term {
    error: Option<String>,
    token: String,
    text: String,
    start_char: u32,
    end_char: u32,
}

impl Term {
    fn new(token: &str, text: String, start_char: u32, end_char: u32) -> Self {
        Self {
            error: None,
            token: String::from(token),
            text,
            start_char,
            end_char,
        }
    }

    fn new_error(message: &str, text: String, start_char: u32, end_char: u32) -> Self {
        Self {
            error: Some(String::from(message)),
            token: String::from("ERROR"),
            text,
            start_char,
            end_char,
        }
    }
}

#[derive(Debug, Clone)]
struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new() -> Self {
        Self { tokens: vec![] }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn lex_input(&self, input: &String) -> Vec<Term> {
        let output = vec![];
        let input_raw: Vec<char> = input.chars().collect();

        output
    }
}

fn main() {
    let example_token: Vec<HashMap<char, u32>> = vec![
        [('a', 1)].iter().cloned().collect(),
        [('b', 2), ('c', 2)].iter().cloned().collect(),
        [('d', 2), ('e', 3)].iter().cloned().collect(),
        [('e', 3)].iter().cloned().collect(),
    ];
    let token = Token::new("EXAMPLE", example_token.into_boxed_slice());
    let mut lexer = Lexer::new();
    lexer.add_token(token);
}

// Example input:
// a(b|c)d*e+
//
// State 0
//  a:      State 1
//  ELSE:   ERROR
// State 1
//  b:      State 2
//  c:      State 2
//  ELSE:   ERROR
// State 2
//  d:      State 2
//  e:      State 3
//  ELSE:   ERROR
// State 3
//  e:      State 3
//  ELSE:   ACCEPT
