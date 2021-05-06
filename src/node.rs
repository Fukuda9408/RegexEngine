use crate::nfa::{Context, NFAFragment};

pub trait Assemble {
    fn assemble(&mut self, context: Context) -> NFAFragment;
}

// Character
//                         ------
//  ------                 ------
// |      |  character   ||      ||
// |  s1  | -----------> ||  s2  ||
// |      |              ||      ||
//  ------                 ------
//                         ------
//  start                  accepts
pub struct Character {
    character: u8
}

impl Character {
    pub fn new(character: u8) -> Self {
        Character {
            character
        }
    }
}

impl Assemble for Character {
    fn assemble(&mut self, context: Context) {
        let s1 = context.new_state();
        let s2 = context.new_state();
        let frag = NFAFragment::new(s1, vec![s2].into_iter().collect());
        frag.connect(s1, self.character, s2)
    }

}

// Union
//                         ------
//  ------                 ------
// |      |  character   ||      ||
// |  s1  | -----------> ||  s2  ||
// |      |              ||      ||
//  ------                 ------
//                         ------
//  start                  accepts
pub struct Union<T>
where
    T: Assemble
{
    operand1: T,
    operand2: T,
}

impl<T> Union<T>
where
    T: Assemble
{
    pub fn new(operand1: T, operand2: T) -> Self {
        Union {
            operand1,
            operand2
        }
    }
}

impl<T> Assemble for Union<T>
where
    T: Assemble
{
    fn assemble(&mut self, context: Context) -> NFAFragment {
        let frag1 = self.operand1.assemble(context);
        let frag2 = self.operand1.assemble(context);
        let flag = frag1.
    }
}
