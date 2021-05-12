use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::dfa::DeterministicFiniteAutomaton;

pub struct NondeterministicFiniteAutomaton
{
    start: i32,
    accept: HashSet<i32>,
    transition: Box<dyn Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>>
}

impl NondeterministicFiniteAutomaton
{
    pub fn new(start: i32, accept: HashSet::<i32>, transition: Box<dyn Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>>) -> Self {
        NondeterministicFiniteAutomaton {
            start,
            accept,
            transition
        }
    }
    pub fn nfa2dfa(self) -> DeterministicFiniteAutomaton
    {
        let start_copy = self.start.clone();
        let expand_start_from_start: HashSet<i32> = self.epsilon_expnad(vec![start_copy].into_iter().collect());
        let accept_copy = self.accept.clone();
        let transition = move |set: HashSet<i32>, character: u8| {
            let mut ret = HashSet::<i32>::new();
            for elem in set {
                match self.trans(elem, Some(character)) {
                    Ok(set) => ret = &ret | &set,
                    Err(_) => continue,
                }
            }
            let res = self.epsilon_expnad(ret);
            return res;
        };

        DeterministicFiniteAutomaton::new(
            expand_start_from_start,
            accept_copy,
            Box::new(transition)
        )
    }

    pub fn trans(&self, state: i32, character: Option<u8>) -> Result<HashSet<i32>, String> {
        (self.transition)(state, character)
    }

    pub fn epsilon_expnad(&self, set: HashSet<i32>) -> HashSet<i32> {
        let mut que: VecDeque<i32> = set.into_iter().collect();
        let mut done = VecDeque::<i32>::new();
        while !que.is_empty() {
            let start = que.pop_back().unwrap();
            done.push_back(start);
            match self.trans(start, None) {
                Ok(nexts) => {
                    for next_state in nexts {
                        if !done.contains(&next_state) {
                            que.push_front(next_state)
                        }
                    }
                },
                Err(_) => continue,
            }
        }
        done.into_iter().collect()
    }
}

#[derive(Debug)]
pub struct NFAFragment {
    pub start: i32,
    pub accepts: HashSet<i32>,
    map: HashMap<(i32, Option<u8>), HashSet<i32>>,
    // (状態、　入力文字)をkeyとし、次の遷移状態の集合を値をして持つ
}

impl NFAFragment
{
    pub fn new(start: i32, accepts: HashSet<i32>) -> Self {
        NFAFragment {
            start,
            accepts,
            map: HashMap::new()
        }
    }

    pub fn connect(&mut self, from: i32, character: Option<u8>, to: i32) {
        match self.map.get_mut(&(from, character)) {
            Some(set) => {
                set.insert(to);
            }
            None => {
                let mut set = HashSet::<i32>::new();
                set.insert(to);
                self.map.insert((from, character), set);
            }
        }
    }

    // startとacceptsは呼び出し側で初期化する必要あり
    pub fn new_skelton(&self) -> Self {
        NFAFragment {
            start: 0,
            accepts: HashSet::new(),
            map: self.map.clone()
        }
    }

    // startとacceptsは呼び出し側で初期化する必要あり
    pub fn or(&self, frag: &NFAFragment) -> Self {
        let mut new_frag = self.new_skelton();
        for (key, value) in frag.new_skelton().map {
            if new_frag.map.contains_key(&key) {
                new_frag.map.get(&key).unwrap().union(&value);
            } else {
                new_frag.map.insert(key, value);
            }
        }
        new_frag
    }

    pub fn build(self) -> NondeterministicFiniteAutomaton
    {
        let accepts_copy = self.accepts.clone();
        let start_copy = self.start;
        let t = move |start: i32, character: Option<u8>| {
                match self.map.get(&(start, character)) {
                    None => return Err("Can't transition".to_string()),
                    Some(value) => return Ok(value.clone())
                }
            };

        NondeterministicFiniteAutomaton::new(
            start_copy,
            accepts_copy,
            Box::new(t)
        )
    }
}

pub struct Context {
    state_count: i32
}

impl Context {
    pub fn new() -> Self {
        Context {
            state_count: 1
        }
    }

    pub fn new_state(&mut self) -> i32 {
        self.state_count += 1;
        self.state_count
    }
}

mod tests {
    #[test]
    fn test_epsilon() {
    }
}
