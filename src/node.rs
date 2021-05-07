use crate::nfa::{Context, NFAFragment};

pub trait Assemble {
    fn assemble(&mut self, context: Context) -> NFAFragment;
}

enum Node {
    Character {
        character: u8,
    },
    Union {
        operand1: Box<Node>,
        operand2: Box<Node>,
    },
    Concat {
        operand1: Box<Node>,
        operand2: Box<Node>,
    },
    Star {
        operand: Box<Node>
    }
}

impl 
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
    fn assemble(&mut self, context: Context) -> NFAFragment{
        let s1 = context.new_state();
        let s2 = context.new_state();
        let frag = NFAFragment::new(s1, vec![s2].into_iter().collect());
        frag.connect(s1, Some(self.character), s2);

        frag
    }

}

// Union
//                                 frag1                  ------
//                                 ------                 ------
//                  ε             |      |  character   ||      ||
//     |--------------------------|  s1  | -----------> ||  s2  ||
//     |                          |      |              ||      ||
//     |                           ------                 ------
//  -------                                               ------
// |       |                       start                  accepts
// |   s   |
// |       |                       frag2                  ------
//  -------                        ------                 ------
//     |                          |      |  character   ||      ||
//     |--------------------------|  s1  | -----------> ||  s2  ||
//                  ε             |      |              ||      ||
//                                 ------                 ------
//                                                        ------
//                                 start                  accepts
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
        let frag = frag1.or(frag2);

        // 新規start
        let start = context.new_state();
        frag.connect(start, None, frag1.start);
        frag.connect(start, None, frag2.start);

        frag.start = start;
        frag.accepts = &frag1.accepts | &frag2.accepts;

        frag
    }
}

// Concat
//                                                              ------
// ------                ------          ------                 ------
//|      |  character   |      |   ε    |      |  character   ||      ||
//|  s1  | -----------> |  s2  |------> |  s1  | -----------> ||  s2  ||
//|      |              |      |        |      |              ||      ||
// ------                ------          ------                 ------
//                                                              ------
// start                 accepts          start                  accepts
pub struct Concat<T>
where
    T: Assemble
{
    operand1: T,
    operand2: T,
}

impl<T> Concat<T>
where
    T: Assemble
{
    pub fn new(operand1: T, operand2: T) -> Self {
        Concat {
            operand1,
            operand2
        }
    }
}

impl<T> Assemble for Concat<T>
where
    T: Assemble
{
    fn assemble(&mut self, context: Context) -> NFAFragment {
        let frag1 = self.operand1.assemble(context);
        let frag2 = self.operand1.assemble(context);
        let frag = frag1.or(frag2);

        for state in frag1.accepts {
            frag.connect(state, None, frag2.start);
        }

        frag.start = frag1.start;
        frag.accepts = frag2.accepts;
        frag
    }
}

// Star
//
//                      ------------------------
//                      |          ε           |
//  ------              ↓                    ------
//  ------            ------                 ------
//||      ||    ε    |      |  character   ||      ||
//||  s1  || ------> |  s1  | -----------> ||  s2  ||
//||      ||         |      |              ||      ||
//  ------            ------                 ------
//  ------                                   ------
// start             start                   accept
pub struct Star<T>
where
    T: Assemble
{
    operand: T
}

impl<T> Star<T>
where
    T: Assemble
{
    pub fn new(operand: T) -> Self {
        Star {
            operand
        }
    }
}
impl<T> Assemble for Star<T>
where
    T: Assemble
{
    fn assemble(&mut self, context: Context) -> NFAFragment {
        let frag_orig = self.operand.assemble(context);
        let frag = frag_orig.new_skelton();

        let start = context.new_state();
        frag.start = start;

        for state in frag_orig.accepts {
            frag.connect(state, None, frag_orig.start);
        }

        frag.connect(start, None, frag_orig.start);
        frag.accepts = &frag_orig.accepts | &vec![start].into_iter().collect();
        frag
    }
}
