use crate::{Action, Matched};
use std::collections::HashMap;

// Ranges are inclusive
#[derive(Debug, Clone)]
pub enum LexerRule {
    // Simple
    Wildcard,
    String(String),
    Range(char, char),

    // Complex
    CountRange(Box<LexerRule>, u32, u32),
    CountRangeEndless(Box<LexerRule>, u32),
    Count(Box<LexerRule>, u32),
    Or(Box<LexerRule>, Box<LexerRule>),
    And(Box<LexerRule>, Box<LexerRule>),
}

impl LexerRule {
    fn get_states_generated(&self) -> usize {
        match self {
            LexerRule::Wildcard => 1,
            LexerRule::String(in_str) => in_str.len(),
            LexerRule::Range(start, end) => char_iter::new(*start, *end).len(),

            // TODO: BE LESS LAZY
            _ => self.generated_fsa(0).len(),
        }
    }

    fn generated_fsa(&self, exit_state: usize) -> Vec<HashMap<Matched, Action>> {
        match self {
            LexerRule::Wildcard => vec![[(Matched::Any, Action::Match(false, exit_state as u32))]
                .iter()
                .cloned()
                .collect()],
            LexerRule::String(in_str) => {
                let mut output = Vec::with_capacity(in_str.len());
                for (i, character) in in_str.chars().enumerate() {
                    output.push(
                        [
                            (
                                Matched::Some(character),
                                Action::Match(false, (exit_state + i) as u32),
                            ),
                            (Matched::Any, Action::Err),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    );
                }
                output
            }
            LexerRule::Range(start, end) => {
                let mut map: HashMap<Matched, Action> =
                    [(Matched::Any, Action::Err)].iter().cloned().collect();
                for character in char_iter::new(*start, *end) {
                    map.insert(
                        Matched::Some(character),
                        Action::Match(false, exit_state as u32),
                    );
                }
                vec![map]
            }

            // TODO:
            //  LexerRule::CountRange(rule, min, max) => {}
            LexerRule::CountRangeEndless(rule, min) => {
                let mut output = LexerRule::Count(rule.to_owned(), *min).generated_fsa(exit_state);
                output.append(
                    &mut rule
                        .as_ref()
                        .generated_fsa(exit_state + output.len() - rule.get_states_generated()),
                );
                let len = output.len();
                output[len - 1].insert(
                    Matched::Any,
                    Action::Match(true, (exit_state + len - 1) as u32),
                );
                output
            }
            LexerRule::Count(rule, count) => {
                if *count == 0 {
                    return vec![];
                }
                let mut output = vec![];
                for _ in 0..*count {
                    output.append(&mut rule.as_ref().generated_fsa(exit_state + output.len()));
                }
                output
            }

            // TODO:
            //  LexerRule::Or(a, b) => {}
            LexerRule::And(a, b) => {
                let mut output = a.as_ref().generated_fsa(exit_state);
                output.append(
                    &mut b
                        .as_ref()
                        .generated_fsa((output.len() + exit_state) as usize),
                );
                output
            }
            _ => unimplemented!(),
        }
    }
}

pub fn generate_fsa_for_token(
    name: &'static str,
    rule: &LexerRule,
) -> Vec<HashMap<Matched, Action>> {
    let mut fsa = rule.generated_fsa(1);
    fsa.push(
        [(Matched::Any, Action::Accept(name))]
            .iter()
            .cloned()
            .collect(),
    );
    fsa
}

pub fn print_fsa(fsa: &Vec<HashMap<Matched, Action>>) {
    for (i, state) in fsa.iter().enumerate() {
        println!("State {}:", i);
        for branch in state {
            println!(
                "  {:?} => {}",
                branch.0,
                match branch.1 {
                    Action::Accept(token_name) => {
                        format!("Accept token \"{}\" and pushback", *token_name)
                    }
                    Action::Match(pushback, next_state) => format!(
                        "Goto {}{}",
                        *next_state,
                        if *pushback { " and pushback" } else { "" }
                    ),
                    Action::Err => String::from("Error"),
                }
            );
        }
    }
}
