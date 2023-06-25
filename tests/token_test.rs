#[test]
fn token_literal_test() {
    use tlang::lexical_analysis::{token::*, token_type::*};

    let sammry = Token::new("0".to_string(), TokenType::DOUBLE, Some(Box::<f64>::new(0.0)), 0);

    assert_eq!("lexeme=0,type=double,line=0", sammry.to_string());
}