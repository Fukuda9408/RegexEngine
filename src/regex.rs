use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::dfa::{DeterministicFiniteAutomaton, DfaRuntime};

use std::collections::HashSet;

pub struct Regex<T>
where
    T: Fn(HashSet<i32>, u8) -> HashSet<i32>
{
    regex: String,
    dfa: DeterministicFiniteAutomaton<T>,
}

impl<T> Regex<T>
where
    T: Fn(HashSet<i32>, u8) -> HashSet<i32>
{
    pub fn new(regex: String) -> Result<Regex<impl Fn(HashSet<i32>, u8) -> HashSet<i32>>, String> {
        let regex_copy = regex.clone();
        let lexer = Lexer::new(regex);
        let mut parser = Parser::new(lexer);
        let nfa = parser.expression()?;
        Ok(
            Regex {
                regex: regex_copy,
                dfa: nfa.nfa2dfa()
            }
        )
    }

    pub fn matches(&self, string: String) -> bool {
        let mut runtime = self.dfa.get_runtime();
        runtime.does_accept(string.as_bytes())
    }
}
