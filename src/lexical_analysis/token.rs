use super::token_type::*;

pub struct Token<T> {
    lexeme: String,        // 词素本身
    token_type: TokenType, // 类型
    literal: Option<T>,    // 字面值
    line: i32,             // 所在行
}

impl<T> Token<T> {
    pub fn new(lexeme: String, token_type: TokenType, literal: Option<T>, line: i32) -> Token<T> {
        Token {
            lexeme,
            literal,
            token_type,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "lexeme={},type={},line={}",
            self.lexeme, self.token_type, self.line
        )
    }
}