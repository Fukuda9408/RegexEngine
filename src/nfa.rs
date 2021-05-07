use std::collections::HashSet;
use std::collections::HashMap;

pub struct NondeterministicFiniteAutomaton<T>
where
    T: Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>
{
    start: i32,
    accept: HashSet<i32>,
    transition: T
}

impl<T> NondeterministicFiniteAutomaton<T>
where
    T: Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>
{
    pub fn new(start: i32, accept: HashSet::<i32>, transition: T) -> Self {
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
        match self.map.get(&(from, character)) {
            Some(&set) => {
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
            map: self.map
        }
    }

    // startとacceptsは呼び出し側で初期化する必要あり
    pub fn or(&self, frag: NFAFragment) -> Self {
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

    fn build(self) -> NondeterministicFiniteAutomaton<impl Fn(i32, Option<u8>) -> Result<HashSet<i32>, String>>
    {
        let accepts_copy = self.accepts.clone();
        let start_copy = self.start;
        let t = move |start: i32, character: Option<u8>| {
                match self.map.get(&(start, character)) {
                    None => return Err("No".to_string()),
                    Some(value) => return Ok(value.clone())
                }
            };

        NondeterministicFiniteAutomaton::new(
            start_copy,
            accepts_copy,
            t
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
