fn main() {
    println!("Hello, world!");
}

//lexer + parser
//lexer stages
// - Scanner
// - evaluator
// for json:
// start token - [ {
// after start token next token must be " , followed by any character until the next ",
// followed by ,
// then again until the stop token ] }
// tokens are separated by ,

// lexer determine what grammar is allowed, or what components are allowed in the grammar
// parser determine semantics, what is allowed to follow each other