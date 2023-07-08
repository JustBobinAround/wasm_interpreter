use std::fs;
use std::io::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Include(Vec<Token>),
    SysCall(String),
    Def(String), // Done
    Insert(String), // Done

    Identifier(Option<String>, String), //expr
    Function(String), // Done
    Call(String, String), //expr
    Bool(bool), //expr
    Integer(i64), //expr
    Float(f64), //expr
    String(String),

    Add, //expr
    Sub, //expr
    Multiply, //expr
    Divide, //expr
    Mod, //expr
    
    Eq, //expr
    Neq, //expr
    Lt,  //expr
    Gt, //expr
    LtEq, //expr
    GtEq, //expr
    Or, //expr
    And, //expr
    Xor, //expr

    Comment(String), //done


    LParen, //expr
    RParen, //expr
    LBrack, //func
    RBrack, //func

    Semicolon, //expr

    Argreg(Option<String>), //done
    Loop(Option<String>, String), //done
    Break, //done
    If(Option<String>, String), //done
    Output(Option<String>, String), //done
    OutputStr, //done
    Input(Option<String>, String), //done?

    Assign(Option<String>, String), //done

    Kill(Option<String>, String), //done -- index Option<String> is not being used for now
    EOF,
    Illegal(usize, String),
    //TODO: serendipity
}

macro_rules! parse {
    (|$self:ident.$peek:ident(), $ch:ident| { $($body:tt)* }) => {
        while let Some($ch) = $self.$peek() {
            match $ch {
                $($body)*
            }
        }
    };
}


pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {

    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_white(&mut self) {
        parse!(|self.peek(), ch| {
            _ => {
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn error(&self, msg: &str) -> String {
        let msg = format!("Error: expected {} at position {}", msg, self.position);
        return msg;
    }

    fn lex_call(&mut self, start: usize, name: String) -> Token {
        let mut token = Token::Illegal(self.position, self.error("an identifier for function arguments"));
        let mut builder = String::new();
        self.skip_white();
        parse!(|self.peek(), ch| {
            ')' => {
                self.advance();
                token = Token::Call(name, builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }

    fn lex_identifier(&mut self, index: Option<String>) -> Token {
        let start_position = self.position;
        let mut token = Token::Illegal(self.position, self.error("an identifier"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            '{' => {
                self.advance();
                if index==None {
                    token = Token::Function(builder);
                }else{
                    token = Token::Illegal(self.position, self.error("a function without a array index"));
                }
                break;
            }
            '(' => {
                self.advance();
                if index==None {
                    token = self.lex_call(start_position, builder); 
                }else{
                    token = Token::Illegal(self.position, self.error("a function without a array index"));
                }
                break;
            }
            '=' => {
                self.advance();
                if let Some(ch) = self.peek() {
                    if ch=='=' || ch=='!' {
                        self.position -= 1;
                        token = Token::Identifier(index, builder);
                    } else {
                        token = Token::Assign(index, builder);
                    }
                }
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    token = Token::Identifier(index, builder);
                    break;
                }
            }
        });
        return token;
    }

    fn lex_number(&mut self) -> Token {
        let start_position = self.position;
        let mut is_float = false;
        let mut builder = String::new();
        parse!(|self.peek(), ch| {
            '.' => {
                self.advance();
                if !is_float {
                    is_float = true;
                    builder.push('.');
                }
            }
            _ => {
                if ch.is_digit(10) {
                    self.advance();
                    builder.push(ch);
                } else {
                    break;
                }
            }
        });

        if is_float {
            let float_number = builder.parse::<f64>().unwrap();
            Token::Float(float_number)
        } else {
            let integer_number = builder.parse::<i64>().unwrap();
            Token::Integer(integer_number)
        }
    }

    // lex luther enjoys the lex_looper function 
    // in the ligma lexer's impl lexer section
    // its 3am rn, what am i doing with my life
    fn lex_looper(&mut self, index: Option<String>) -> Token {
        let mut token = Token::Illegal(self.position, self.error("some sort of loop or break statement"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            '{' => {
                self.advance();
                token = Token::Loop(index, builder);
                break;
            }
            ';' => {
                self.advance();
                token = Token::Break;
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }

    fn lex_comment(&mut self, pos: usize) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a comment ending escape"));
        let mut builder = String::new();
        
        parse!(|self.peek(), ch| {
            '*' => {
                self.advance();
                if let Some(ch) = self.peek(){
                    if ch == '/' {
                        self.advance();
                        token = Token::Comment(builder);
                        break;
                    }
                }
                builder.push(ch);
            }
            _ => {
                self.advance();
                builder.push(ch);
            }
        });
        return token;
    }

    fn lex_div(&mut self) -> Token {
        let mut token = Token::Divide;

        parse!(|self.peek(), ch| {
            '*' => {
                let pos = self.position;
                self.advance();
                token = self.lex_comment(pos);
                break;
            }
            _ => {break;}
        });
        return token;
    }

    fn lex_if(&mut self, index: Option<String>) -> Token {
        let start_position = self.position;
        let mut token = Token::Illegal(self.position, self.error("a expression or secondary ?"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            '{' => {
                self.advance();
                token = Token::If(index, builder);
                break;
            }
            '?' => {
                self.advance();
                token = self.lex_input(index); 
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }

    fn lex_string(&mut self) -> Token {
        let start_position = self.position;
        let mut token = Token::Illegal(self.position, self.error("balenced string quotes"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            '\"' => {
                self.advance();
                token = Token::String(builder);
                break;
            }
            '\\' => {
                self.advance();
                if let Some(ch) = self.peek() {
                    let esc: char;
                    match ch {
                        '\"' => {esc='\"'}
                        'n' => {esc='\n'}
                        't' => {esc='\t'}
                        'r' => {esc='\r'}
                        _ => {esc=ch;}
                    }
                    builder.push(esc);
                    self.advance();
                } else {
                    token = Token::Illegal(self.position, self.error("a valid escape sequence"));
                    break;
                }
            }
            _ => {
                builder.push(ch);
                self.advance();
            }
        });
        return token;
    }

    fn lex_output(&mut self) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a identifier"));
        let mut builder = String::new();
        let mut index:Option<String> = None;

        
        parse!(|self.peek(), ch| {
            '[' => {
                self.advance();
                index = self.lex_array();
                continue;
            }
            '\"' => {
                token = Token::OutputStr;
                break;
            }
            ';' => {
                self.advance();
                token = Token::Output(index, builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@'{
                    builder.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;

    }
    fn lex_input(&mut self, index: Option<String>) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a identifier"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            ';' => {
                self.advance();
                token = Token::Input(index, builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@'{
                    builder.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }
    fn lex_logic(&mut self) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a logical expressiong"));

        parse!(|self.peek(), ch| {
            '=' => {
                self.advance();
                token = Token::Eq;
                break;
            }
            '!' => {
                self.advance();
                token = Token::Neq;
                break;
            }
            _ => {break;}
        });
        return token;
    }

    fn lex_lt(&mut self) -> Token {
        let start = self.position;
        let mut token = Token::Lt;
        parse!(|self.peek(), ch| {
            '=' => {
                self.advance();
                token = Token::LtEq;
                break;
            }
            _ => {break;}
        });
        
        return token;
    }

    fn lex_gt(&mut self) -> Token {
        let start = self.position;
        let mut token = Token::Gt;
        parse!(|self.peek(), ch| {
            '=' => {
                self.advance();
                token = Token::GtEq;
                break;
            }
            _ => {break;}
        });
        
        return token;
    }

    fn lex_def(&mut self) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a def name"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            '>' => {
                self.advance();
                token = Token::Def(builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() {
                    builder.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }

    fn lex_insert(&mut self) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a def name"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            ')' => {
                self.advance();
                token = Token::Insert(builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() {
                    builder.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }
    fn lex_syscall(&mut self) -> Token {
        let start = self.position;
        let mut token = Token::Illegal(self.position, self.error("a syscall"));
        let mut builder = String::new();
        parse!(|self.peek(), ch| {
            '#' => {
                self.advance();
                if builder=="T"{
                    token = Token::Bool(true);
                } else if builder=="F" {
                    token = Token::Bool(false);
                } else {
                    token = Token::SysCall(builder);
                }
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch == '_'{
                    self.advance();
                    builder.push(ch);
                } else {
                    break;
                }

            }
        });
        return token;
    }
    pub fn read_file(file_path: String) -> Result<String, Error> {
        return fs::read_to_string(file_path);
    }

    fn lex_include(&mut self) -> Token {
        let start = self.position;
        let mut builder = String::new();
        parse!(|self.peek(), ch| {
            ']' => {
                self.advance();
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='/' || ch == '_' || ch == '.' {
                    builder.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        });

        let mut tokens: Vec<Token> = Vec::new();
        match Lexer::read_file(builder) {
            Ok(file_contents) => {
                let mut lexer = Lexer::new(&file_contents);
                loop {
                    let token = lexer.lex_next_token();
                    if token == Token::EOF {
                        break;
                    }
                    tokens.push(token);
                }

            }
            Err(_) => {}
        }
        let token = Token::Include(tokens);
        return token;
    }

    fn lex_macro(&mut self) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a def or a boolean"));

        parse!(|self.peek(), ch| {
            '(' => {
                self.advance();
                token = self.lex_insert();
                break;
            }
            '<' => {
                self.advance();
                token = self.lex_def();
                break;
            }
            '[' => {
                self.advance();
                token = self.lex_include();
                break;
            }
            _ => {
                if ch.is_whitespace() {
                    self.advance();
                } else if ch.is_alphanumeric() {
                    token = self.lex_syscall();
                    break;
                } else {
                    break;
                }

            }
        });
        return token;
    }

    fn lex_array(&mut self) -> Option<String> {
        let mut index: Option<String> = None;
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            ']' => {
                self.advance();
                index = Some(builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });

        return index;
    } 

    fn lex_kill(&mut self, index: Option<String>) -> Token {
        let mut token = Token::Illegal(self.position, self.error("a valid variable"));
        let mut builder = String::new();

        parse!(|self.peek(), ch| {
            ';' => {
                self.advance();
                token = Token::Kill(index, builder);
                break;
            }
            _ => {
                if ch.is_alphanumeric() || ch=='_' || ch=='@' {
                    self.advance();
                    builder.push(ch);
                } else if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
        });
        return token;
    }

    pub fn lex_next_token(&mut self) -> Token {
        let mut token = Token::EOF;
        let mut index: Option<String> = None;

        parse!(|self.peek(), ch| {
            '\"' => {
                self.advance();
                token = self.lex_string();
                break;
            }
            '#' => {
                self.advance();
                token = self.lex_macro();
                break;
            }
            '=' => {
                self.advance();
                token = self.lex_logic();
                break;
            }
            '<' => {
                self.advance();
                token = self.lex_lt();
                break;
            }
            '>' => {
                self.advance();
                token = self.lex_gt();
                break;
            }
            '|' => {
                self.advance();
                token = Token::Or;
                break;
            }
            '&' => {
                self.advance();
                token = Token::And;
                break;
            }
            '^' => {
                self.advance();
                token = Token::Xor;
                break;
            }
            '+' => {
                self.advance();
                token = Token::Add;
                break;
            }
            '-' => {
                self.advance();
                token = Token::Sub;
                break;
            }
            '*' => {
                self.advance();
                if let Some(ch) = self.peek() {
                    if ch == '*' {
                        self.advance();
                        token = self.lex_kill(index);
                        break;
                    }
                }
                token = Token::Multiply;
                break;
            }
            '/' => {
                self.advance();
                token = self.lex_div();
                break;
            }
            '%' => {
                self.advance();
                token = Token::Mod;
                break;
            }
            '(' => {
                self.advance();
                token = Token::LParen;
                break;
            }
            ')' => {
                self.advance();
                token = Token::RParen;
                break;
            }
            '{' => {
                self.advance();
                token = Token::LBrack;
                break;
            }
            '}' => {
                self.advance();
                token = Token::RBrack;
                break;
            }
            '[' => {
                self.advance();
                index = self.lex_array();
                continue;
            }
            ';' => {
                self.advance();
                token = Token::Semicolon;
                break;
            }
            '@' => {
                self.advance();
                self.skip_white();
                if let Some(ch) = self.peek() {
                    if ch == '=' {
                        self.advance();
                    }
                }
                token = Token::Argreg(index);
                break;
            }
            '?' => {
                self.advance();
                token = self.lex_if(index);
                break;
            }
            '!' => {
                self.advance();
                token = self.lex_output();
                break;
            }
            '$' => {
                self.advance();
                token = self.lex_looper(index);
                break;
            }

            _ => {
                if ch.is_whitespace(){
                    self.advance();
                } else if ch.is_digit(10) {
                    token = self.lex_number();
                    break;
                } else if ch.is_alphanumeric() || ch=='_' || ch =='@' {
                    token = self.lex_identifier(index);
                    break;
                } else { self.advance();
                    token = Token::Illegal(self.position, self.error("a valid token"));
                    break;
                }
            }

        });
        return token;
    }
}

