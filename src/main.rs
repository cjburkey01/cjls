use lex::LexerRule;
use std::collections::HashMap;

pub mod lex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action {
    Match(u32),           // Contains next state
    Accept(&'static str), // Contains matched token name
    Err,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Matched {
    Some(char),
    Any,
}

#[derive(Debug, Clone)]
pub struct Term {
    error: Option<char>,
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

    fn new_error(error: char, text: String, start_char: u32, end_char: u32) -> Self {
        Self {
            error: Some(error),
            token: String::from("ERROR"),
            text,
            start_char,
            end_char,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    fsa: Box<[HashMap<Matched, Action>]>,
}

impl Lexer {
    pub fn new(fsa: Vec<HashMap<Matched, Action>>) -> Self {
        Self {
            fsa: fsa.into_boxed_slice(),
        }
    }

    pub fn lex_input(&self, input: &String) -> Vec<Term> {
        let mut output = vec![];
        let mut input_raw: Vec<char> = input.chars().collect();

        let mut current_state = 0u32;
        let mut current_token: Vec<char> = vec![];
        let mut start_char = 0u32;
        let mut current_char = 0u32;
        'lexer_loop: while input_raw.len() > 0 {
            let current_state_branches = self
                .fsa
                .get(current_state as usize)
                .expect(format!("invalid state: {}", current_state).as_str());

            let next_char = input_raw.remove(0);
            current_token.push(next_char);

            let next_action = match current_state_branches.get(&Matched::Some(next_char)) {
                Some(action) => action,
                None => current_state_branches.get(&Matched::Any).expect(
                    format!("invalid input '{}' for state {}", next_char, current_state).as_str(),
                ),
            };
            match next_action {
                Action::Match(next_state) => current_state = *next_state,
                Action::Accept(token_name) => {
                    Self::reset_token(
                        Option::Some(*token_name),
                        &mut output,
                        &mut current_state,
                        &mut current_token,
                        &mut start_char,
                        current_char,
                        &mut input_raw,
                    );
                }
                Action::Err => {
                    Self::reset_token(
                        Option::None,
                        &mut output,
                        &mut current_state,
                        &mut current_token,
                        &mut start_char,
                        current_char,
                        &mut input_raw,
                    );
                    println!("Invalid token '{}' in state 0", next_char);
                    break 'lexer_loop;
                }
            }

            current_char += 1;
        }

        output
    }

    fn reset_token(
        token: Option<&str>,
        output: &mut Vec<Term>,
        current_state: &mut u32,
        current_token: &mut Vec<char>,
        start_char: &mut u32,
        current_char: u32,
        input_raw: &mut Vec<char>,
    ) {
        input_raw.insert(0, current_token.pop().unwrap());
        output.push(match token {
            Some(token_name) => Term::new(
                token_name,
                current_token.iter().collect(),
                *start_char,
                current_char,
            ),
            None => Term::new_error(
                input_raw[0],
                current_token.iter().collect(),
                *start_char,
                current_char,
            ),
        });

        current_token.clear();
        *current_state = 0;
        *start_char = current_char;
    }
}

// Example input:
//  TOKEN:   a(b|c)d*e+
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
//  ELSE:   ACCEPT "TOKEN"
fn main() {
    let example_fsa: Vec<&[(Matched, Action)]> = vec![
        // State 0
        &[
            (Matched::Some('a'), Action::Match(1)),
            (Matched::Any, Action::Err),
        ],
        // State 1
        &[
            (Matched::Some('b'), Action::Match(2)),
            (Matched::Some('c'), Action::Match(2)),
            (Matched::Any, Action::Err),
        ],
        // State 2
        &[
            (Matched::Some('d'), Action::Match(2)),
            (Matched::Some('e'), Action::Match(3)),
            (Matched::Any, Action::Err),
        ],
        // State 3
        &[
            (Matched::Some('e'), Action::Match(3)),
            (Matched::Any, Action::Accept("EXAMPLE_INPUT")),
        ],
    ];

    //    {
    //        let lexer = Lexer::new(nice_fsa_to_raw_fsa(example_fsa));
    //        let output = lexer.lex_input(&String::from("abdeeabeeacddddeeacfe"));
    //        println!("{:#?}", output);
    //    }
    {
        //        let example_rule = LexerRule::And(
        //            Box::new(LexerRule::Count(Box::new(LexerRule::Range('a', 'c')), 2)),
        //            Box::new(LexerRule::And(
        //                Box::new(LexerRule::String(String::from("b"))),
        //                Box::new(LexerRule::And(
        //                    Box::new(LexerRule::String(String::from("c"))),
        //                    Box::new(LexerRule::Wildcard),
        //                )),
        //            )),
        //        );
        let example_rule = LexerRule::And(
            Box::new(LexerRule::Or(
                Box::new(LexerRule::String(String::from("a"))),
                Box::new(LexerRule::String(String::from("b"))),
            )),
            Box::new(LexerRule::Count(Box::new(LexerRule::Range('a', 'c')), 2)),
        );
        println!("{:#?}", example_rule);
        lex::print_fsa(&lex::generate_fsa_for_token("A_B", &example_rule));
    }
}

fn nice_fsa_to_raw_fsa(nice_fsa: Vec<&[(Matched, Action)]>) -> Vec<HashMap<Matched, Action>> {
    let mut raw_fsa: Vec<HashMap<Matched, Action>> = vec![];
    for state in nice_fsa {
        let mut branch_state_map = HashMap::new();
        for branch in state {
            branch_state_map.insert(branch.0, branch.1);
        }
        raw_fsa.push(branch_state_map);
    }
    raw_fsa
}
