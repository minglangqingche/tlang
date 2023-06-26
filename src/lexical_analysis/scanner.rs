use std::collections::HashMap;
use super::{token::*, token_type::*};
use crate::chunk::value::*;

pub struct Scanner {
    code: Vec<char>,
    token_list: Vec<Token>,

    start: usize, // 被扫描词素的第一个字符
    current: usize, // 被处理的当前字符
    line: u32, // 被扫描词素所在行
    keywords: HashMap<Vec<char>, TokenType>,

    error: u32,
}

impl Scanner {
    pub fn new(code: String) -> Self {
        let mut keywords: HashMap<Vec<char>, TokenType> = HashMap::new();
        keywords.insert(vec!['l', 'e', 't'], TokenType::LET);
        keywords.insert(vec!['f', 'n'], TokenType::FN);
        keywords.insert(vec!['c', 'a', 'l', 's', 's'], TokenType::CLASS);
        keywords.insert(vec!['f', 'a', 'l', 's', 'e'], TokenType::FALSE);
        keywords.insert(vec!['t', 'r', 'u', 'e'], TokenType::TRUE);
        keywords.insert(vec!['t', 'h', 'i', 's'], TokenType::THIS);
        keywords.insert(vec!['e', 'l', 's', 'e'], TokenType::ELSE);
        keywords.insert(vec!['i', 'f'], TokenType::IF);
        keywords.insert(vec!['f', 'o', 'r'], TokenType::FOR);
        keywords.insert(vec!['w', 'h', 'i', 'l', 'e'], TokenType::WHILE);
        keywords.insert(vec!['n', 'u', 'l', 'l'], TokenType::NULL);
        keywords.insert(vec!['p', 'r', 'i', 'n', 't'], TokenType::PRINT);
        keywords.insert(vec!['r', 'e', 't', 'u', 'r', 'n'], TokenType::RETURN);
        keywords.insert(vec!['s', 'u', 'p', 'e', 'r'], TokenType::SUPER);

        Self {
            code: code.chars().collect(),
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
        self.token_list.push(Token::new(vec!['E', 'O', 'F'], TokenType::EOF, None, self.line));

        if self.error != 0 {
            Err(self.error)
        }else {
            Ok(&(self.token_list))
        }
    }

    fn get_token(&mut self) -> Option<Token> {
        let c = self.advance();
        match c {
            // 单字符匹配
            '(' => Some(Token::new([c].to_vec(), TokenType::LEFT_PAREN, None, self.line)),
            ')' => Some(Token::new([c].to_vec(), TokenType::RIGHT_PAREN, None, self.line)),
            '{' => Some(Token::new([c].to_vec(), TokenType::LEFT_BRACE, None, self.line)),
            '}' => Some(Token::new([c].to_vec(), TokenType::RIGHT_BRACE, None, self.line)),
            '.' => Some(Token::new([c].to_vec(), TokenType::DOT, None, self.line)),
            ',' => Some(Token::new([c].to_vec(), TokenType::COMMA, None, self.line)),
            '+' => Some(Token::new([c].to_vec(), TokenType::PLUS, None, self.line)),
            '*' => Some(Token::new([c].to_vec(), TokenType::STAR, None, self.line)),
            '-' => Some(Token::new([c].to_vec(), TokenType::MINUS, None, self.line)),
            ';' => Some(Token::new([c].to_vec(), TokenType::SEMICOLON, None, self.line)),

            // 单字符或双字符匹配
            '!' => {
                if self.match_next('=') {
                    Some(Token::new([c].to_vec(), TokenType::BANG_EQUAL, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::BANG, None, self.line))
                }
            },
            '<' => {
                if self.match_next('=') {
                    Some(Token::new([c].to_vec(), TokenType::LESS_EQUAL, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::LESS, None, self.line))
                }
            },
            '>' => {
                if self.match_next('=') {
                    Some(Token::new([c].to_vec(), TokenType::GREATER_EQUAL, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::GREATER, None, self.line))
                }
            },
            '=' => {
                if self.match_next('=') {
                    Some(Token::new([c].to_vec(), TokenType::EQUAL_EQUAL, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::EQUAL, None, self.line))
                }
            },
            '&' => {
                if self.match_next('&') {
                    Some(Token::new([c].to_vec(), TokenType::AND_AND, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::AND, None, self.line))
                }
            },
            '|' => {
                if self.match_next('|') {
                    Some(Token::new([c].to_vec(), TokenType::OR_OR, None, self.line))
                }else {
                    Some(Token::new([c].to_vec(), TokenType::OR, None, self.line))
                }
            },

            // 单行注释及slash
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                    None
                }else if self.match_next('*') { // 多行注释
                    self.multiline_comment()
                }else {
                    Some(Token::new([c].to_vec(), TokenType::SLASH, None, self.line))
                }
            },

            // 空白字符
            ' ' => None,
            '\r' => None,
            '\t' => None,
            // 换行符
            '\n' => {
                self.line += 1;
                None
            },

            // string
            '"' => {
                self.get_string()
            },

            _ => {
                if Self::is_digit(c) {
                    self.get_digit()
                }else if Self::is_alpha(c) {
                    self.identifier()
                }else {
                    self.error("unknown char!", self.line, &c.to_string())
                }
            },
        }
        
    }

    fn multiline_comment(&mut self) -> Option<Token> {
        while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }else if self.peek() == '/' && self.peek_next() == '*' {
                // 吸收 start 与 slash
                self.advance();
                self.advance();

                self.multiline_comment();
            }

            self.advance();
        }
        // 吸收 start 与 slash
        self.advance();
        self.advance();

        None
    }

    fn identifier(&mut self) -> Option<Token> {
        while Self::is_digit_or_alpha(self.peek()) {
            self.advance();
        }

        // 获取标识符
        let a = &self.code[self.start..self.current];
        let a_type = self.keywords.get(a);
        Some(Token::new(a.to_vec(),
            match a_type { Some(&val) => val , None => TokenType::IDENTIFIER, },
            None, self.line))
    }

    fn is_digit_or_alpha(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' // 英文
        || (c >= '\u{4e00}' && c <= '\u{9fa5}')
    }

    fn get_digit(&mut self) -> Option<Token> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance(); // 吸收dot

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        
        let val = &self.code[self.start..self.current];
        let t: String = val.to_vec().iter().collect();
        Some(Token::new(val.to_vec(), TokenType::DOUBLE, Some(Value::Double(t.parse::<f64>().unwrap())), self.line))
    }

    fn peek_next(&self) -> char {
        if self.is_end() || self.current+1 >= self.code.len() {
            '\0'
        }else {
            // &self.code[self.current+1..self.current+2]
            self.code[self.current + 1]
        }
    }

    fn get_string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return self.error("Unterminated string", self.line, "\"..");
        }

        self.advance(); // 读入结尾引号

        // 去除引号
        let t = self.code[self.start+1..self.current-1].to_vec();
        Some(Token::new(t.to_vec(), TokenType::STRING, Some(Value::String(t.iter().collect())), self.line))
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        }else {
            self.code[self.current]
        }
    }

    fn error(&mut self, massege: &str, line:u32, val: &str) -> Option<Token> {
        crate::interpreter_error::error(&format!("{}\n in line={}, char={}", massege, line, val));
        self.error += 1;
        Some(Token::new(vec![], TokenType::ERROR, None, line))
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_end() {
            false
        }else if self.peek() == expected {
            self.current += 1;
            true
        }else {
            false
        }
    }

    fn advance(&mut self) -> char {
        if self.is_end() {
            '\0'
        }else {
            self.current += 1;
            self.code[self.current - 1]
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.code.len()
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advance_test() {
        let mut a = Scanner::new(String::from("0123456789"));
        assert_eq!(a.advance(), '0');
        assert_eq!(a.advance(), '1');
    }

    #[test]
    fn peek_test() {
        let mut a = Scanner::new("0123".to_string());
        a.advance(); // 0
        a.advance(); // 1
        assert_eq!(a.current, 2);
        assert_eq!(a.peek(), '2'); // 1
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
        assert_eq!(a.to_string(), "lexeme=['h', 'e', 'l', 'l', 'o'],type=String,line=1".to_string());
    }

    #[test]
    fn is_digit_test() {
        assert_eq!(Scanner::is_digit('\0'), false);
        assert_eq!(Scanner::is_digit('0'), true);
        assert_eq!(Scanner::is_digit('7'), true);
        assert_eq!(Scanner::is_digit('a'), false);
    }

    #[test]
    fn is_alpha_test() {
        assert_eq!(Scanner::is_alpha('a'), true);
        assert_eq!(Scanner::is_alpha('g'), true);
        assert_eq!(Scanner::is_alpha('Z'), true);
        assert_eq!(Scanner::is_alpha('0'), false);
    }

    fn get_a_token<'a>(t: TokenType) -> Token {
        Token::new(vec![], t, None, 0)
    }
}
