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
    pub fn get_states_generated(&self) -> usize {
        match self {
            LexerRule::Wildcard => 1,
            LexerRule::String(in_str) => in_str.len(),
            LexerRule::Range(start, end) => char_iter::new(*start, *end).len(),

            // TODO: BE LESS LAZY
            _ => self.generated_fsa(0).len(),
        }
    }

    pub fn generated_fsa(&self, initial_state: usize) -> Vec<HashMap<Matched, Action>> {
        match self {
            LexerRule::Wildcard => {
                vec![[(Matched::Any, Action::Match((initial_state + 1) as u32))]
                    .iter()
                    .cloned()
                    .collect()]
            }
            LexerRule::String(in_str) => {
                let mut output = Vec::with_capacity(in_str.len());
                for (i, character) in in_str.chars().enumerate() {
                    output.push(
                        [
                            (
                                Matched::Some(character),
                                Action::Match((initial_state + i + 1) as u32),
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
                        Action::Match((initial_state + 1) as u32),
                    );
                }
                vec![map]
            }
            // TODO: THIS PROBABLY DOESN'T WORK BUT I'M TOO LAZY TO FIX IT NOW
            LexerRule::CountRange(rule, min, max) => {
                if *min == *max {
                    return LexerRule::Count(*rule, *min).generated_fsa(initial_state);
                }
                let mut output = Vec::new();
                let end = (rule.get_states_generated() * (max * 2 - min)) as usize;
                for i in 0..*max {
                    output.append(&mut rule.generated_fsa(initial_state + output.len() - 1));
                    if i >= *min {
                        output.append(&mut rule.generated_fsa(end - 1));
                    }
                }
                output
            }
            LexerRule::CountRangeEndless(rule, min) => {}
            LexerRule::Count(rule, count) => {
                if *count == 1 {
                    return rule.as_ref().generated_fsa(initial_state);
                }
            }
            LexerRule::Or(a, b) => {}
            LexerRule::And(a, b) => {}
        }
    }
}

pub fn generate_fsa(tokens: Box<[(&str, LexerRule)]>) -> Vec<HashMap<Matched, Action>> {
    let mut output = vec![];
    for token in tokens.iter() {}
    output
}

pub fn generate_fsa_for_token(name: &str, rule: LexerRule) -> Vec<HashMap<Matched, Action>> {
    let mut output = vec![];

    output
}
