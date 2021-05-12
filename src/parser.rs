use crate::lexer::{Token, TokenKind, Lexer};
use crate::node::Node;
use crate::nfa::{NondeterministicFiniteAutomaton, Context};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    look: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            look: Token::new(0x00, TokenKind::INITIALIZE)
        };
        // Tokenを一つ読み込む
        parser.move_parser();
        parser
    }

    pub fn match_parse(&mut self, tag: TokenKind) -> Result<(), String> {
        if self.look.kind != tag {
            return Err("syntax error".to_string())
        }
        self.move_parser();
        Ok(())
    }
    pub fn move_parser(&mut self) {
        self.look = self.lexer.scan()
    }

    // expression -> subexpr EOF
    // subexpr -> seq '|' subexpr | seq
    // seq -> subseq | ''
    // subseq -> star subseq | star
    // star -> factor '*' | factor
    // factor -> '(' subexpr ')' | CHARACTER
    fn factor(&mut self) -> Result<Node, String>
    {
        // println!("factor:{:?}", self);
        if self.look.kind == TokenKind::LPAREN {
            self.match_parse(TokenKind::LPAREN)?;
            let node = self.subexpr()?;
            self.match_parse(TokenKind::RPAREN)?;
            Ok(node)
        } else {
            let node = Node::character(self.look.value);
            self.match_parse(TokenKind::CHACTER)?;
            Ok(node)
        }
    }

    fn star(&mut self) -> Result<Node, String>
    {
        let node = self.factor()?;
        // println!("star:{:?}", self);
        if self.look.kind == TokenKind::OPE_STAR {
            self.match_parse(TokenKind::OPE_STAR)?;
            return Ok(Node::star(node))
        }
        return Ok(node)
    }

    fn seq(&mut self) -> Result<Node, String> {
        // println!("seq:{:?}", self);
        if self.look.kind == TokenKind::LPAREN || self.look.kind == TokenKind::CHACTER {
            return self.subseq();
        } else {
            return Ok(Node::character(0x00));
        }
    }

    fn subseq(&mut self) -> Result<Node, String> {
        let node1 = self.star()?;
        // println!("subseq:{:?}", self);
        if self.look.kind == TokenKind::LPAREN || self.look.kind == TokenKind::CHACTER {
            let node2 = self.subseq()?;
            let node = Node::concat(node1, node2);
            return Ok(node)
        } else {
            return Ok(node1);
        }
    }

    fn subexpr(&mut self) -> Result<Node, String> {
        let node = self.seq()?;
        // println!("subexpr:{:?}", self);
        if self.look.kind == TokenKind::OPE_UNION {
            self.match_parse(TokenKind::OPE_UNION)?;
            let node2 = self.subexpr()?;
            return Ok(Node::union(node, node2))
        }
        return Ok(node);
    }

    pub fn expression(&mut self) -> Result<NondeterministicFiniteAutomaton, String>
    {
        let node = self.subexpr()?;
        // println!("expression:{:?}", self);
        self.match_parse(TokenKind::EOF)?;

        // NFAの作成
        let mut context = Context::new();
        println!("{:?}", node);
        let fragment = node.assemble(&mut context);
        println!("fragment: {:?}", fragment);
        Ok(fragment.build())
    }
}
