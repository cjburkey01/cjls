use std::collections::HashMap;

/// The next action that the lexer will take depending on the current state and its branches
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Action<'a> {
    /// Change the current state to the provided state without changing the current character
    Reset(usize),

    /// Add the current character into the current token buffer, pull the next character from the input, and change
    /// the current state to the provided state
    Match(usize),

    /// Emit the current token buffer as a token with the provided token name without changing the current character,
    /// and reset the state to state 0
    Accept(&'a str),

    /// Add the current character into the current token buffer, emit an error token, and reset the state to state 0
    Error,
}

/// The current character's match relative to the current state
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Match {
    /// The current character matches a specific character branch of the current state
    Char(char),

    /// The current character does not match a specific character branch of the current state, this is a fallback
    Other,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    name: String,
    start: usize,
    value: String,
    error: bool,
}

impl Token {
    fn new_raw(name: &str, lexer_state: &mut LexerState, error: bool) -> Self {
        Self {
            name: name.to_string(),
            start: lexer_state.char_pos - lexer_state.current_token.len() - 1,
            value: lexer_state.current_token.iter().collect(),
            error,
        }
    }

    fn new(name: &str, lexer_state: &mut LexerState) -> Self {
        Self::new_raw(name, lexer_state, false)
    }

    fn new_error(lexer_state: &mut LexerState) -> Self {
        Self::new_raw("ERROR", lexer_state, true)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct LexerState {
    input_raw: Vec<char>,
    current_char: Option<char>,
    current_token: Vec<char>,
    current_state: usize,
    char_pos: usize,
    output: Vec<Token>,
    finished: bool,
}

impl LexerState {
    fn new(input: &str) -> Self {
        Self {
            input_raw: input.chars().collect(),
            current_char: Option::None,
            current_token: vec![],
            current_state: 0,
            char_pos: 0,
            output: vec![],
            finished: false,
        }
    }

    fn reset(&mut self) {
        self.current_token.clear();
        self.current_state = 0;
        //        self.next_char_no_inc();
    }

    fn next_char_no_inc(&mut self) {
        self.current_char = if self.input_raw.len() == 0 {
            Option::None
        } else {
            Option::Some(self.input_raw.remove(0))
        };
    }

    fn next_char(&mut self) {
        self.next_char_no_inc();
        self.char_pos += 1;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Lexer<'a> {
    states: Box<[HashMap<Match, Action<'a>>]>,
}

impl<'a> Lexer<'a> {
    pub fn create(states: Box<[HashMap<Match, Action<'a>>]>) -> Self {
        Self { states }
    }

    pub fn new(states: &[&[(Match, Action<'a>)]]) -> Self {
        let mut states_vec = vec![];
        for state in states {
            states_vec.push(state.iter().cloned().collect());
        }
        Self::create(states_vec.into_boxed_slice())
    }

    pub fn lex_input(&self, input: &str) -> Vec<Token> {
        let mut lexer_state = LexerState::new(input);
        lexer_state.next_char();

        while !lexer_state.finished {
            match self.states.get(lexer_state.current_state) {
                Option::Some(state) => self.process_state(&mut lexer_state, state),
                Option::None => panic!(format!(
                    "Error: no state {} found in lexer with {} states",
                    lexer_state.current_state,
                    self.states.len(),
                )),
            }
        }

        lexer_state.output
    }

    fn process_state(&self, lexer_state: &mut LexerState, state: &HashMap<Match, Action<'a>>) {
        let next_action = match state.get(&match lexer_state.current_char {
            Option::Some(current_char) => Match::Char(current_char),
            Option::None => {
                lexer_state.finished = true;
                Match::Other
            }
        }) {
            Option::Some(action) => action,
            Option::None => state.get(&Match::Other).unwrap_or(&Action::Error),
        };

        match next_action {
            Action::Reset(next_state) => self.on_reset(lexer_state, *next_state),
            Action::Match(next_state) => self.on_match(lexer_state, *next_state),
            Action::Accept(token_name) => self.on_accept(lexer_state, *token_name),
            Action::Error => self.on_error(lexer_state),
        }
    }

    #[inline]
    fn on_reset(&self, lexer_state: &mut LexerState, next_state: usize) {
        println!(
            "reset from state {} to state {}",
            lexer_state.current_state, next_state
        );

        lexer_state.current_state = next_state;
    }

    #[inline]
    fn on_match(&self, lexer_state: &mut LexerState, next_state: usize) {
        println!(
            "match '{}' in state {} and reset to state {}",
            match lexer_state.current_char {
                Option::Some(current_char) => current_char.to_string(),
                Option::None => String::from("EOI"),
            },
            lexer_state.current_state,
            next_state
        );

        match lexer_state.current_char {
            Option::Some(current_char) => lexer_state.current_token.push(current_char),
            _ => {}
        }

        lexer_state.current_state = next_state;
        lexer_state.next_char();
    }

    #[inline]
    fn on_accept(&self, lexer_state: &mut LexerState, token_name: &str) {
        println!(
            "accept \"{}\" and reset to state 1",
            Iterator::collect::<String>(lexer_state.current_token.iter()),
        );

        let new_token = Token::new(token_name, lexer_state);
        lexer_state.output.push(new_token);
        lexer_state.reset();
    }

    #[inline]
    fn on_error(&self, lexer_state: &mut LexerState) {
        println!(
            "Error: unexpected '{}' in state {}",
            match lexer_state.current_char {
                Option::Some(current_char) => current_char.to_string(),
                Option::None => String::from("EOI"),
            },
            lexer_state.current_state,
        );

        if lexer_state.current_state == 0 {
            // If an error occurs in state 0, no recovery is possible (yet)
            lexer_state.finished = true;
        }

        let error_token = Token::new_error(lexer_state);
        lexer_state.output.push(error_token);
        lexer_state.reset();
    }
}

fn main() {
    println!("Goodnight moon");

    // Example lexer
    {
        let states: &[&[(Match, Action)]] = &[
            // 0
            &[(Match::Char('h'), Action::Match(1))],
            // 1
            &[(Match::Char('e'), Action::Match(2))],
            // 2
            &[(Match::Char('y'), Action::Match(3))],
            // 3
            &[(Match::Other, Action::Accept("TOKEN_HEY"))],
        ];

        let lexer = Lexer::new(states);
        println!("{:#?}", lexer.lex_input("heyheyy"));
    }
}
