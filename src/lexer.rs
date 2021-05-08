#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum TokenKind {
    CHACTER,
    OPE_UNION,
    OPE_STAR,
    LPAREN,
    RPAREN,
    EOF,
    INITIALIZE,
}

#[derive(Debug)]
pub struct Token {
    pub value: u8,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(value: u8, kind: TokenKind) -> Self {
        Token {
            value,
            kind
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    string_list: String
}

impl Lexer {
    pub fn new(string_list: String) -> Self {
        Lexer {
            string_list: string_list.chars().rev().collect()
        }
    }

    pub fn scan(&mut self) -> Token {
        let ch = self.string_list.pop();
        match ch {
            None => return Token::new(0xff, TokenKind::EOF),
            Some(c) => {
                match c as u8 {
                    b'\\' => {
                        return Token::new(self.string_list.pop().unwrap() as u8, TokenKind::CHACTER)
                    },
                    b'|' => return Token::new(c as u8, TokenKind::OPE_UNION),
                    b'*' => return Token::new(c as u8, TokenKind::OPE_STAR),
                    b'(' => return Token::new(c as u8, TokenKind::LPAREN),
                    b')' => return Token::new(c as u8, TokenKind::RPAREN),
                    _ => return Token::new(c as u8, TokenKind::CHACTER),
                }
            }
        }
    }
}
