use std::collections::HashSet;

pub struct DeterministicFiniteAutomaton
{
    start: HashSet<i32>,
    accept: HashSet<i32>,
    transition: Box<dyn Fn(HashSet<i32>, u8) -> HashSet<i32>>,
}

impl DeterministicFiniteAutomaton
{
    pub fn new(start: HashSet<i32>, accept: HashSet<i32>, transition: Box<dyn Fn(HashSet<i32>, u8) -> HashSet<i32>>) -> Self {
        DeterministicFiniteAutomaton {
            start,
            accept,
            transition
        }
    }

    pub fn get_runtime(&self) -> DfaRuntime {
        DfaRuntime::new(self)
    }

    pub fn trans(&self, state: HashSet<i32>, character: u8) -> HashSet<i32> {
        (self.transition)(state, character)
    }
}

pub struct DfaRuntime<'a>
{
    dfa: &'a DeterministicFiniteAutomaton,
    cur_state: HashSet<i32>
}

impl<'a> DfaRuntime<'a>
{
    pub fn new(dfa: &'a DeterministicFiniteAutomaton) -> Self {
        let cur_state = dfa.start.clone();
        DfaRuntime {
            dfa,
            cur_state: cur_state
        }
    }

    pub fn do_trantision(&mut self, character: u8) {
        let cur_state = self.cur_state.clone();
        self.cur_state =  self.dfa.trans(cur_state, character)
    }

    // 現在の状態と受理状態の積をとりからでなければ、
    // 現在の状態に受理状態となる状態があるということになるため
    // 現在の状態は受理される
    // NFA -> DFAとしたときの受理状態は正しくは
    // 「受理状態を含む状態の集合」が正しいがNFAのままでの受理状態を
    // 使用する
    pub fn is_accept_state(&self) -> bool {
        !(&self.dfa.accept & &self.cur_state).is_empty()
    }

    pub fn does_accept(&mut self, input: &[u8]) -> bool {
        for &alphabet in input.iter() {
            println!("before: current_state: {:?}", self.cur_state);
            self.do_trantision(alphabet);
            println!("after: current_state: {:?}", self.cur_state);
        }
        self.is_accept_state()
    }
}
