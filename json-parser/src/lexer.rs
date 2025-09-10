
//simple json lexer that describes the json grammar

use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    StartBraces(char), //{
    EndBraces(char), //}
    Comma(char), //,
    Colon(char), //:
    StartBracket(char), //[
    EndBracket(char), //]
    Quote(char), //"
    String(&'static str),
    Number(f64),
    True, // true
    False, // false
    Null // null
}

static DELIMETER_TOKENS_MAP: [Option<Token>; 128] = {
    let mut map: [Option<Token>; 128] = [None;128];
    map[b'{' as usize] = Some(Token::StartBraces('{'));
    map[b'}' as usize] = Some(Token::EndBraces('}'));
    map[b'[' as usize] = Some(Token::StartBracket('['));
    map[b']' as usize] = Some(Token::EndBracket(']'));
    map[b',' as usize] = Some(Token::Comma(','));
    map[b':' as usize] = Some(Token::Colon(':'));
    map[b'"' as usize] = Some(Token::Quote('"'));

    map
};


// const fn create_map() -> HashMap<char, Token> {
//     HashMap::from([
//         ('{', Token::StartBraces('{')),
//         ('}', Token::EndBraces('{')),
//         ('[', Token::StartBracket('[')),
//         (']', Token::EndBracket(']')),
//         (',', Token::Comma(',')),
//         (':', Token::Colon(':')),
//         ('"', Token::Quote('"')),
//     ])
// }

fn tokenize(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    // if input is empty, return an error
    if input.is_empty() {
        return Err("Input is empty".into());
    }

    // if input does not start with start token, return error

    let mut tokens: Vec<Token> = Vec::new();

    // check if character is a delimiter, then add to tokens

    // else if {"me " : 2}


    Ok(tokens)
}

mod tests {
    use std::string::ParseError;
    use crate::lexer::Token::{EndBraces, StartBraces};
    use crate::lexer::tokenize;

    #[test]
    fn can_tokenize_simple_json() {
        // assert_eq!(tokenize("{}"), Ok(vec![StartBraces('{'), EndBraces('}')]));
        // assert_eq!(tokenize("[]"), Ok(vec![]));
        // assert_eq!(tokenize("454"), Err(ParseError::from("Invalid start token. Valid json must start with { or [")));
    }
}