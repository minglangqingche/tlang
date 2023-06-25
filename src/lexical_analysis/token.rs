use std::any::Any;
use super::token_type::*;

#[derive(Debug)]
pub struct Token {
    lexeme: String, // 词素本身
    token_type: TokenType, // 类型
    val: Option<Box<dyn Any>>, // 字面值
    line: u32, // 所在行
}

impl Token {
    pub fn new(lexeme: String, token_type: TokenType, val: Option<Box<dyn Any>>, line: u32) -> Token {
        Token {
            lexeme,
            val,
            token_type,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("lexeme={},type={},line={}", self.lexeme, self.token_type, self.line)
    }
}

impl Eq for Token {}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}
