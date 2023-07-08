mod utils;

use wasm_bindgen::prelude::*;

mod ligma;
use ligma::{
    fun::Fun as Fun,
    lexer::Lexer as Lexer,
    lexer::Token as Token,
    expr::Expr as Expr,
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn ligma(input: String) -> String{
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(&input);
    loop {
        let token = lexer.lex_next_token();
        match token {
            Token::EOF => {
                break;
            }
            Token::Include(to_include) => {
                for token_to_include in to_include {
                    tokens.push(token_to_include);
                }
            }
            _ => {
                tokens.push(token);
            }
        }
        
    }
    let mut fun = Fun::new(tokens);
    fun.eval();
    let mut output = String::from("L I G M A  Interprets Generally Meaningless Abstractions\nv0.1.0\n\n");
    let printer = fun.get_output();
    {
        let printer = printer.lock().unwrap();
        output.push_str(&printer);
    }

    return output;
}

