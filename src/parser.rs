use crate::lexer::{Token, TokenKind, Lexer};
pub enum NodeKind {
    Union,
    Star,
    Concat
}

pub enum Node {

}

pub struct Parser {
    lexer: Lexer,
    look: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            look: Token::new(0x00, TokenKind::INITIALIZE)
        }
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
    fn factor(&mut self) -> Result<Node, String> {
        if self.look.kind == TokenKind::LPAREN {
            self.match_parse(TokenKind::LPAREN)?;
            let node = self.subexpr()?;
            self.match_parse(TokenKind::RPAREN)?;
            Ok(node)
        } else {
            let node = Character(self.look.value);
            self.match_parse(TokenKind::CHACTER);
            Ok(node)
        }
    }

    fn star(&mut self) -> Result<Node, String> {
        let node = self.factor()?;
        if self.look.kind == TokenKind::OPE_STAR {
            self.match_parse(TokenKind::OPE_STAR)?;
            node = Star(node)
        }
        return Ok(node)
    }

    fn seq(&mut self) -> Result<Node, String> {
        if self.look.kind == TokenKind::LPAREN || self.look.kind == TokenKind::CHACTER {
            return self.subexpr();
        } else {
            return Character(0x00);
        }
    }

    fn subseq(&mut self) -> Result<Node, String> {
        let node1 = self.star()?;
        if self.look.kind == TokenKind::LPAREN || self.look.kind == TokenKind::CHACTER {
            let node2 = self.subseq()?;
            let node = Concat(node1, node2);
            return Ok(node)
        } else {
            return Ok(node1);
        }
    }

    fn subexpr(&mut self) -> Result<Node, String> {
        let node = self.seq()?;
        if self.look.kind == TokenKind::OPE_UNION {
            self.match_parse(TokenKind::OPE_UNION)?;
            let node2 = self.subexpr()?;
            let node = Union(node, node2);
        }
        return Ok(node);
    }

    fn expression(&mut self) -> Result<Node, String> {
        let node = self.subexpr()?;
        self.match_parse(TokenKind::EOF)?;

        // NFAの作成
    }
}
