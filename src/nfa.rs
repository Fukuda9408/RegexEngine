use std::collections::HashSet;

pub struct NondeterministicFiniteAutomaton {
    start: i32,
    accept: HashSet<i32>,
    transition: fn(i32, Option<u8>) -> Result<HashSet::<i32>, String>,
}

impl NondeterministicFiniteAutomaton {
    pub fn new(start: i32, accept: HashSet::<i32>, transition: fn(i32, Option<u8>) -> Result<HashSet::<i32>, String>) -> Self {
        NondeterministicFiniteAutomaton {
            start,
            accept,
            transition
        }
    }

    pub fn trans(&self, state: i32, character: Option<u8>) -> Result<HashSet<i32>, String> {
        (self.transition)(state, character)
    }
}

pub struct NFAFragment {
    start: i32,
    accepts: HashSet<i32>,
    map: 
}
pub struct Context {
    state_count: i32
}

impl Context {
    pub fn new() -> Self {
        Context {
            state_count: 0
        }
    }

    pub fn new_state(&mut self) -> i32 {
        self.state_count += 1;
        self.state_count
    }
}
