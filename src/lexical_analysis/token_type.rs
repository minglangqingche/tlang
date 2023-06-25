use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // 单字符
    // ( )
    LEFT_PAREN, RIGHT_PAREN,
    // { }
    LEFT_BRACE, RIGHT_BRACE,
    // , . - +
    COMMA, DOT, MINUS, PLUS,
    // ; / *
    SEMICOLON, SLASH, STAR,

    // 单字符或双字符
    // ! !=
    BANG, BANG_EQUAL,
    // = ==
    EQUAL, EQUAL_EQUAL,
    // > >=
    GREATER, GREATER_EQUAL,
    // < <=
    LESS, LESS_EQUAL,
    // & &&
    AND, AND_AND,
    // | ||
    OR, OR_OR,

    // 文字
    // 标识符 字符串 双精度浮点
    IDENTIFIER, STRING, DOUBLE,

    // 关键字
    CLASS, ELSE, FALSE, FN, FOR, IF, NULL, PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,

    EOF,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            Self::LET => String::from("let"),
            Self::WHILE => String::from("while"),

            Self::THIS => String::from("this"),
            Self::TRUE => String::from("true"),

            Self::RETURN => String::from("return"),
            Self::SUPER => String::from("super"),

            Self::NULL => String::from("null"),
            Self::PRINT => String::from("print"),

            Self::FALSE => String::from("false"),
            Self::FN => String::from("fn"),

            Self::FOR => String::from("for"),
            Self::IF => String::from("if"),

            Self::CLASS => String::from("class"),
            Self::ELSE => String::from("else"),

            Self::IDENTIFIER => String::from("identifier"),
            Self::STRING => String::from("String"),
            Self::DOUBLE => String::from("double"),

            Self::AND => String::from("&"),
            Self::OR => String::from("|"),

            Self::LESS => String::from("<"),
            Self::LESS_EQUAL => String::from("<="),

            Self::EQUAL => String::from("="),
            Self::EQUAL_EQUAL => String::from("=="),

            Self::GREATER => String::from(">"),
            Self::GREATER_EQUAL => String::from(">="),

            Self::BANG => String::from("!"),
            Self::BANG_EQUAL => String::from("!="),

            Self::SEMICOLON => String::from(";"),
            Self::SLASH => String::from("/"),
            Self::STAR => String::from("*"),

            Self::LEFT_PAREN => String::from("("),
            Self::RIGHT_PAREN => String::from(")"),

            Self::LEFT_BRACE => String::from("{"),
            Self::RIGHT_BRACE => String::from("}"),

            Self::COMMA => String::from(","),
            Self::DOT => String::from("."),
            Self::MINUS => String::from("-"),
            Self::PLUS => String::from("+"),

            Self::AND_AND => String::from("&&"),
            Self::OR_OR => String::from("||"),

            Self::EOF => String::from("EOF"),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::LET => 0,
            Self::WHILE => 1,

            Self::THIS => 2,
            Self::TRUE => 3,

            Self::RETURN => 4,
            Self::SUPER => 5,

            Self::NULL => 6,
            Self::PRINT => 7,

            Self::FALSE => 8,
            Self::FN => 9,

            Self::FOR => 10,
            Self::IF => 11,

            Self::CLASS => 12,
            Self::ELSE => 13,

            Self::IDENTIFIER => 14,
            Self::STRING => 15,
            Self::DOUBLE => 16,

            Self::AND => 17,
            Self::OR => 18,

            Self::LESS => 19,
            Self::LESS_EQUAL => 20,

            Self::EQUAL => 21,
            Self::EQUAL_EQUAL => 22,

            Self::GREATER => 23,
            Self::GREATER_EQUAL => 24,

            Self::BANG => 25,
            Self::BANG_EQUAL => 26,

            Self::SEMICOLON => 27,
            Self::SLASH => 28,
            Self::STAR => 29,

            Self::LEFT_PAREN => 30,
            Self::RIGHT_PAREN => 31,

            Self::LEFT_BRACE => 32,
            Self::RIGHT_BRACE => 33,

            Self::COMMA => 34,
            Self::DOT => 35,
            Self::MINUS => 36,
            Self::PLUS => 37,

            Self::AND_AND => 38,
            Self::OR_OR => 39,

            Self::EOF => 40,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Eq for TokenType {}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self.to_u32() == other.to_u32()
    }
}

impl Copy for TokenType {}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        assert_eq!(TokenType::AND, TokenType::AND);
        assert_ne!(TokenType::AND, TokenType::SEMICOLON);
    }
}
