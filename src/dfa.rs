use std::collections::HashSet;

pub struct DeterministicFiniteAutomaton {
    start: i32,
    accept: HashSet<i32>,
    transition: fn(i32, u8) -> i32,
}

impl DeterministicFiniteAutomaton {
    pub fn new(start: i32, accept: HashSet<i32>, transition: fn(i32, u8) -> i32) -> Self {
        DeterministicFiniteAutomaton {
            start,
            accept,
            transition
        }
    }

    pub fn get_runtime(&self) -> DfaRuntime {
        DfaRuntime::new(self)
    }

    pub fn trans(&self, state: i32, character: u8) -> i32 {
        (self.transition)(state, character)
    }
}

pub struct DfaRuntime<'a> {
    dfa: &'a DeterministicFiniteAutomaton,
    cur_state: i32
}

impl<'a> DfaRuntime<'a> {
    pub fn new(dfa: &'a DeterministicFiniteAutomaton) -> Self {
        let cur_state = dfa.start;
        DfaRuntime {
            dfa,
            cur_state: cur_state
        }
    }

    pub fn do_trantision(&mut self, character: u8) {
        self.cur_state =  self.dfa.trans(self.cur_state, character)
    }

    pub fn is_accept_state(&self) -> bool {
        self.dfa.accept.contains(&self.cur_state)
    }

    pub fn doea_accept(&mut self, input: &[u8]) -> bool {
        for &alphabet in input.iter() {
            self.do_trantision(alphabet);
        }
        self.is_accept_state()
    }
}
