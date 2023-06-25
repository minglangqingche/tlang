use super::{token::*, token_type::*};

pub struct Scanner {
    code: String,
    token_list: Vec<Token>,

    start: usize, // 被扫描词素的第一个字符
    current: usize, // 被处理的当前字符
    line: u32, // 被扫描词素所在行

    error: u32,
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Self {
            code,
            token_list: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error: 0,
        }
    }

    pub fn scann(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            // scanning token
            self.start = self.current;
            match self.get_token() {
                None => {},
                Some(val) => {
                    self.token_list.push(val);
                }
            }
        }
        
        // the end of code
        self.token_list.push(Token::new(String::from("EOF"), TokenType::EOF, None, 0));

        &(self.token_list)
    }

    fn get_token(&mut self) -> Option<Token> {
        let c = self.advance();
        match &c[..] {
            // 单字符匹配
            "(" => Some(Token::new(c, TokenType::LEFT_PAREN, None, self.line)),
            ")" => Some(Token::new(c, TokenType::RIGHT_PAREN, None, self.line)),
            "{" => Some(Token::new(c, TokenType::LEFT_BRACE, None, self.line)),
            "}" => Some(Token::new(c, TokenType::RIGHT_BRACE, None, self.line)),
            "." => Some(Token::new(c, TokenType::DOT, None, self.line)),
            "," => Some(Token::new(c, TokenType::COMMA, None, self.line)),
            "+" => Some(Token::new(c, TokenType::PLUS, None, self.line)),
            "*" => Some(Token::new(c, TokenType::STAR, None, self.line)),
            "-" => Some(Token::new(c, TokenType::MINUS, None, self.line)),
            ";" => Some(Token::new(c, TokenType::SEMICOLON, None, self.line)),

            // 单字符或双字符匹配
            "!" => {
                if self.match_next("=") {
                    Some(Token::new(c, TokenType::BANG_EQUAL, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::BANG, None, self.line))
                }
            },
            "<" => {
                if self.match_next("=") {
                    Some(Token::new(c, TokenType::LESS_EQUAL, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::LESS, None, self.line))
                }
            },
            ">" => {
                if self.match_next("=") {
                    Some(Token::new(c, TokenType::GREATER_EQUAL, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::GREATER, None, self.line))
                }
            },
            "=" => {
                if self.match_next("=") {
                    Some(Token::new(c, TokenType::EQUAL_EQUAL, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::EQUAL, None, self.line))
                }
            },
            "&" => {
                if self.match_next("&") {
                    Some(Token::new(c, TokenType::AND_AND, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::AND, None, self.line))
                }
            },
            "|" => {
                if self.match_next("|") {
                    Some(Token::new(c, TokenType::OR_OR, None, self.line))
                }else {
                    Some(Token::new(c, TokenType::OR, None, self.line))
                }
            },

            // 注释及/
            "/" => {
                if self.match_next("/") {
                    while self.peek().ne("\n") && !self.is_end() {
                        self.advance();
                    }
                    None
                }else {
                    Some(Token::new(c, TokenType::SLASH, None, self.line))
                }
            },

            // 空白字符
            " " => None,
            "\r" => None,
            "\t" => None,
            // 换行符
            "\n" => {
                self.line += 1;
                None
            },

            _ => {
                self.error()
            },
        }
        
    }

    fn peek(&self) -> &str {
        if self.is_end() {
            "\0"
        }else {
            &self.code[self.current..self.current+1]
        }
    }

    fn error(&mut self) -> Option<Token> {
        crate::interpreter_error::error("unknown char!");
        self.error += 1;
        None
    }

    fn match_next(&mut self, expected: &str) -> bool {
        if self.is_end() {
            false
        }else if self.code[self.current..self.current+1].eq(expected) {
            self.current += 1;
            true
        }else {
            false
        }
    }

    fn advance(&mut self) -> String {
        self.current += 1;
        self.code[self.current - 1..self.current].to_string()
    }

    fn is_end(&self) -> bool {
        self.current >= self.code.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advance_test() {
        let mut a = Scanner::new(String::from("0123456789"));
        assert_eq!(a.advance(), "0");
        assert_eq!(a.advance(), "1");
    }

    #[test]
    fn peek_test() {
        let mut a = Scanner::new("//00000\na".to_string());
        a.get_token();
        assert_eq!(a.advance(), "\n");
    }

    #[test]
    fn get_token_test() {
        let mut a = Scanner::new("//00000".to_string());
        assert_eq!(a.get_token(), None);
        let mut a = Scanner::new("/000".to_string());
        assert_eq!(a.get_token().unwrap(), get_a_token(TokenType::SLASH));
    }

    fn get_a_token(t: TokenType) -> Token {
        Token::new(String::new(), t, None, 0)
    }
}
