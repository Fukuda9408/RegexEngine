use crate::nfa::{Context, NFAFragment};

pub trait Assemble {
    fn assemble(&mut self, context: Context) -> NFAFragment;
}

#[derive(Debug)]
pub enum Node {
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

impl Node {
    pub fn character(character: u8) -> Self {
        Node::Character {
            character
        }
    }

    pub fn union(operand1: Node, operand2: Node) -> Self {
        Node::Union {
            operand1: Box::new(operand1),
            operand2: Box::new(operand2),
        }
    }

    pub fn concat(operand1: Node, operand2: Node) -> Self {
        Node::Concat {
            operand1: Box::new(operand1),
            operand2: Box::new(operand2),
        }
    }

    pub fn star(operand: Node) -> Self {
        Node::Star {
            operand: Box::new(operand),
        }
    }

    pub fn assemble(self, context: &mut Context) -> NFAFragment {
        match self {

//                         ------
//  ------                 ------
// |      |  character   ||      ||
// |  s1  | -----------> ||  s2  ||
// |      |              ||      ||
//  ------                 ------
//                         ------
//  start                  accepts
            Node::Character {
                character
            } => {
                let s1 = context.new_state();
                let s2 = context.new_state();
                let mut frag = NFAFragment::new(s1, vec![s2].into_iter().collect());
                frag.connect(s1, Some(character), s2);

                frag
            },

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
            Node::Union {
                operand1,
                operand2
            } => {
                let frag1 = operand1.assemble(context);
                let frag2 = operand2.assemble(context);
                let mut frag = frag1.or(&frag2);

                // 新規start
                let start = context.new_state();
                frag.connect(start, None, frag1.start);
                frag.connect(start, None, frag2.start);

                frag.start = start;
                frag.accepts = &frag1.accepts | &frag2.accepts;

                frag
            },

// Concat
//                                                              ------
// ------                ------          ------                 ------
//|      |  character   |      |   ε    |      |  character   ||      ||
//|  s1  | -----------> |  s2  |------> |  s1  | -----------> ||  s2  ||
//|      |              |      |        |      |              ||      ||
// ------                ------          ------                 ------
//                                                              ------
// start                 accepts          start                  accepts
            Node::Concat {
                operand1,
                operand2
            } => {
                let frag1 = operand1.assemble(context);
                let frag2 = operand2.assemble(context);
                let mut frag = frag1.or(&frag2);

                for state in frag1.accepts {
                    frag.connect(state, None, frag2.start);
                }

                frag.start = frag1.start;
                frag.accepts = frag2.accepts;
                frag
            },

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
            Node::Star {
                operand
            } => {
                let frag_orig = operand.assemble(context);
                let mut frag = frag_orig.new_skelton();

                let start = context.new_state();
                frag.start = start;

                for state in frag_orig.accepts.iter() {
                    frag.connect(*state, None, frag_orig.start);
                }

                frag.connect(start, None, frag_orig.start);
                frag.accepts = &frag_orig.accepts | &vec![start].into_iter().collect();
                frag
            }
        }
    }
}
