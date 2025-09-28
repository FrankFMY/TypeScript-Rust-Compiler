use crate::lexer::Lexer;

pub fn test_lexer() {
    let code =
        "function calculate(a: number, b: number, operation: string): number { return a + b; }";
    println!("Input code: '{}'", code);
    println!("Code length: {}", code.len());

    for (i, ch) in code.chars().enumerate() {
        println!("Position {}: '{}'", i, ch);
    }

    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().unwrap();

    for token in tokens {
        println!("{:?}", token);
    }
}
