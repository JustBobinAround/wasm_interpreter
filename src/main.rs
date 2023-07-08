mod ligma;
use std::io;
use ligma::{
    fun::Fun as Fun,
    lexer::Lexer as Lexer,
    lexer::Token as Token,
    expr::Expr as Expr,
};
use std::panic;
use std::env;

fn main() {
    let mut tokens: Vec<Token> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    if let Some(opt) = args.get(2) {
        debug = opt == "debug";
    }
    if let Some(input_file) = args.get(1) {
        match Lexer::read_file(input_file.to_string()) {
            Ok(file_contents) => {
                let mut lexer = Lexer::new(&file_contents);
                loop {
                    let token = lexer.lex_next_token();
                    match token {
                        Token::EOF => {
                            break;
                        }
                        Token::Include(to_include) => {
                            for token_to_include in to_include {
                                if debug {
                                    println!("{:?}", token_to_include);
                                }
                                tokens.push(token_to_include);
                            }
                        }
                        _ => {
                            if debug {
                                println!("{:?}", token);
                            }
                            tokens.push(token);
                        }
                    }
                    
                }
            }
            Err(_) => {
                println!("file not found");

            }
        }
    } else {
        println!("expected a file");
    }

    if !debug{
        let result = panic::catch_unwind(|| {
            let mut fun = Fun::new(tokens);
            fun.eval(false);
        });
        if let Err(_) = result {
            println!("Well that happened.. this shouldn\'t be possible. Send me what your program is");
            // Handle the stack overflow here
        }
    }


}

