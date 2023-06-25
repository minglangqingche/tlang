use std::collections::HashMap;
use super::{token::*, token_type::*};

pub struct Scanner<'a> {
    code: String,
    token_list: Vec<Token>,

    start: usize, // 被扫描词素的第一个字符
    current: usize, // 被处理的当前字符
    line: u32, // 被扫描词素所在行
    keywords: HashMap<&'a str, TokenType>,

    error: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(code: String) -> Self {
        let mut keywords: HashMap<&'a str, TokenType> = HashMap::new();
        keywords.insert("let", TokenType::LET);
        keywords.insert("fn", TokenType::FN);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("if", TokenType::IF);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("while", TokenType::WHILE);
        keywords.insert("null", TokenType::NULL);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);

        Self {
            code,
            token_list: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error: 0,
            keywords,
        }
    }

    pub fn scann(&mut self) -> Result<&Vec<Token>, u32> {
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
        self.token_list.push(Token::new(String::from("EOF"), TokenType::EOF, None, self.line));

        if self.error != 0 {
            Err(self.error)
        }else {
            Ok(&(self.token_list))
        }
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

            // 单行注释及slash
            "/" => {
                if self.match_next("/") {
                    while self.peek().ne("\n") && !self.is_end() {
                        self.advance();
                    }
                    None
                }else if self.match_next("*") { // 多行注释
                    while self.peek().ne("*") && self.peek_next().ne("/") && !self.is_end() {
                        if self.peek().eq("\n") {
                            self.line += 1;
                        }

                        self.advance();
                    }
                    // 吸收 start 与 slash
                    self.advance();
                    self.advance();

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

            // string
            "\"" => {
                self.get_string()
            },

            _ => {
                if Self::is_digit(&c[..]) {
                    self.get_digit()
                }else if Self::is_alpha(&c[..]) {
                    self.identifier()
                }else {
                    self.error("unknown char!", self.line, &c)
                }
            },
        }
        
    }

    fn identifier(&mut self) -> Option<Token> {
        while Self::is_digit_or_alpha(self.peek()) {
            self.advance();
        }

        let a = &self.code[self.start..self.current]; // 获取标识符
        let a_type = self.keywords.get(a);
        Some(Token::new(a.to_string(),
            match a_type { Some(&val) => val , None => TokenType::IDENTIFIER, },
            None, self.line))
    }

    fn is_digit_or_alpha(c: &str) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn is_alpha(c: &str) -> bool {
        let c = c.as_bytes()[0];
        (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
    }

    fn get_digit(&mut self) -> Option<Token> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek().eq(".") && Self::is_digit(self.peek_next()) {
            self.advance(); // 吸收dot

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        
        let val = &self.code[self.start..self.current];
        Some(Token::new(val.to_string(), TokenType::DOUBLE, Some(Box::<f64>::new(val.parse::<f64>().unwrap())), self.line))
    }

    fn peek_next(&self) -> &str {
        if self.is_end() || self.current+1 >= self.code.len() {
            "\0"
        }else {
            &self.code[self.current+1..self.current+2]
        }
    }

    fn get_string(&mut self) -> Option<Token> {
        while self.peek().ne("\"") && !self.is_end() {
            if self.peek().eq("\n") {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            self.error("Unterminated string", self.line, "\"..");
            return None;
        }

        self.advance(); // 读入结尾引号

        let string = self.code[self.start+1..self.current-1].to_string(); // 去除引号
        Some(Token::new(string.clone(), TokenType::STRING, Some(Box::<String>::new(string)), self.line))
    }

    fn peek(&self) -> &str {
        if self.is_end() {
            "\0"
        }else {
            &self.code[self.current..self.current+1]
        }
    }

    fn error(&mut self, massege: &str, line:u32, val: &str) -> Option<Token> {
        crate::interpreter_error::error(&format!("{}\n in line={}, char={}", massege, line, val));
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
        if self.is_end() {
            "\0".to_string()
        }else {
            self.current += 1;
            self.code[self.current - 1..self.current].to_string()
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.code.len()
    }

    fn is_digit(x: &str) -> bool {
        let c = x.as_bytes()[0];
        c >= b'0' && c <= b'9'
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
        let mut a = Scanner::new("0123".to_string());
        a.advance(); // 0
        a.advance(); // 1
        assert_eq!(a.current, 2);
        assert_eq!(a.peek(), "2"); // 1
        assert_eq!(a.current, 2);

        let mut a = Scanner::new("//23456\n01".to_string());
        a.get_token();
        a.get_token();
        assert_eq!(a.line, 2);
    }

    #[test]
    fn get_token_test() {
        let mut a = Scanner::new("//00000".to_string());
        assert_eq!(a.get_token(), None);
        let mut a = Scanner::new("/000".to_string());
        assert_eq!(a.get_token().unwrap(), get_a_token(TokenType::SLASH));
    }

    #[test]
    fn get_string_test() {
        let mut a = Scanner::new(r#""hello""#.to_string());
        a.advance();
        let a = a.get_string();
        assert_eq!(a, Some(get_a_token(TokenType::STRING)));
        let a = a.unwrap();
        assert_eq!(a.to_string(), "lexeme=hello,type=String,line=1".to_string());
    }

    #[test]
    fn is_digit_test() {
        assert_eq!(Scanner::is_digit("\0"), false);
        assert_eq!(Scanner::is_digit("0"), true);
        assert_eq!(Scanner::is_digit("7"), true);
        assert_eq!(Scanner::is_digit("a"), false);
    }

    #[test]
    fn is_alpha_test() {
        assert_eq!(Scanner::is_alpha("a"), true);
        assert_eq!(Scanner::is_alpha("g"), true);
        assert_eq!(Scanner::is_alpha("Z"), true);
        assert_eq!(Scanner::is_alpha("0"), false);
    }

    fn get_a_token(t: TokenType) -> Token {
        Token::new(String::new(), t, None, 0)
    }
}
