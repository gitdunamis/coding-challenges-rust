mod lexer;

fn analyse(input: &str) -> u8 {
    0
}


mod tests {
    use crate::analyse;

    #[test]
    fn can_parse_simple_json() {
        assert_eq!(analyse("{}"), 0);
        assert_eq!(analyse(""), 1);
    }
}