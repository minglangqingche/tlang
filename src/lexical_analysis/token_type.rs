use std::fmt::Display;

#[allow(non_camel_case_types)]
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
    // && ||
    AND, OR, 

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

            Self::IDENTIFIER => String::from("标识符"),
            Self::STRING => String::from("字符串"),
            Self::DOUBLE => String::from("数字"),

            Self::AND => String::from("&&"),
            Self::OR => String::from("||"),

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

            Self::EOF => String::from("EOF"),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}