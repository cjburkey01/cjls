use crate::{Action, Matched};
use std::collections::HashMap;

pub enum LexerFrag {
    Wildcard,
    String(String),
    Range(char, char),
    CountRange(Box<LexerFrag>, u32, u32),
    CountRangeEndless(Box<LexerFrag>, u32),
    Count(Box<LexerFrag>, u32),
    Or(Box<LexerFrag>, Box<LexerFrag>),
}

pub struct LexerRule {
    frags: Vec<LexerFrag>,
}

impl LexerRule {
    pub fn new(frags: Vec<LexerFrag>) -> Self {
        Self { frags }
    }
}

pub fn generate_fsa(rules: Vec<LexerRule>) -> Vec<HashMap<Matched, Action>> {}
