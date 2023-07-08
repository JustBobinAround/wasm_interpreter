use crate::Expr;

use std::sync::{Arc, Mutex};
use std::io;
use crate::Token;
use std::collections::HashMap;
macro_rules! parse {
    (|$self:ident.$peek:ident(), $token:ident| { $($body:tt)* }) => {
        while let Some($token) = $self.$peek() {
            match $token {
                $($body)*
            }
        }
    };
}
//Stackoverflows shouldn't be possible now unless your computer sucks?
//Any sub function or expression should only evaluate if under the limit
const RECURS_LIMIT: i64 = 300;


// Don't ask me why I chose such a terrible data structure. I don't know and I don't want to
// rewrite at this point. 
//
// ArcArg is a static return register
// ArcFuns is a static register of all past declared functions
// ArcDefs is a static register of all past defined macros
// VarList is a private variable list only accessable to the current function
type ArcArg = Arc<Mutex<HashMap<Option<String>, Expr>>>;
type ArcFuns = Arc<Mutex<HashMap<String, Fun>>>;
type ArcDefs = Arc<Mutex<HashMap<String, Vec<Token>>>>;
type VarList = HashMap<String, HashMap<Option<String>, Expr>>;
// I want to embed in wasm, so no stack overflow allowed :(
type SoMonitor = Arc<Mutex<i64>>;
type Printer = Arc<Mutex<String>>;

#[derive(Debug, Clone)]
pub struct Fun {
    defs: ArcDefs,
    argreg: ArcArg,
    functions: ArcFuns,
    tokens: Vec<Token>,
    vars: VarList,    
    position: usize,
    loop_start: Vec<usize>,
    paren_counter: Vec<i64>,
    stack: SoMonitor,
    output: Printer,
}


// Perfect naming conventions
impl Fun {
    fn mod_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Int(t % b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Float(t % b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                let mut tt = format!("{}",t);
                for _c in b.chars(){
                    tt.pop();
                }
                a = Expr::String(tt);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn div_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Int(t / b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Float(t / b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_integer(b);
                let mut cloned_t = t.clone();
                let mut tt = String::new();
                for _c in 0..b{
                    if let Some(ch) = cloned_t.pop() {
                        tt.push(ch);
                    }
                }
                a = Expr::String(tt);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }

    fn mult_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t && b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Int(t * b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Float(t * b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_integer(b);
                let mut tt = format!("{}",t);
                for _c in 0..b{
                    tt = format!("{}{}",tt,t);
                }
                a = Expr::String(tt);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn sub_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Int(t - b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Float(t - b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_integer(b);
                let mut tt = format!("{}",t);
                for _c in 0..b{
                    tt.pop();
                }
                a = Expr::String(tt);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }

    fn add_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t || b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Int(t + b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Float(t + b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::String(format!("{}{}",b,t));
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn eq_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t == b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t == b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t == b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t == b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn neq_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t != b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t != b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t != b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t != b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn lt_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t < b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t < b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t < b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t < b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn gt_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t > b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t > b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t > b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t > b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn lteq_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t <= b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t <= b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t <= b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t <= b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn gteq_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t >= b);
            }
            Expr::Int(t) => {
                let b = Expr::parse_exp_integer(b);
                a = Expr::Bool(t >= b);
            }
            Expr::Float(t) => {
                let b = Expr::parse_exp_float(b);
                a = Expr::Bool(t >= b);
            }
            Expr::String(t) => {
                let b = Expr::parse_exp_string(b);
                a = Expr::Bool(t >= b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn or_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t || b);
            }
            Expr::Int(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t || b);
            }
            Expr::Float(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t || b);
            }
            Expr::String(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t || b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn and_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t && b);
            }
            Expr::Int(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t && b);
            }
            Expr::Float(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t && b);
            }
            Expr::String(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t && b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn xor_exp(&mut self, exp_a: Expr, exp_b: Expr) -> Expr {
        let a: Expr;
        let b: Expr;
        match exp_b {
            Expr::BinOp(_,_,_) => {
                b = self.eval_binop(exp_b);
            }
            _ => {
                b = exp_b;
            }
        }
        match exp_a {
            Expr::BinOp(_,_,_) => {
                a = self.eval_binop(exp_a);
            }
            Expr::Bool(t) => {
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::Int(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::Float(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            Expr::String(_) => {
                let t = Expr::parse_exp_boolean(exp_a);
                let b = Expr::parse_exp_boolean(b);
                a = Expr::Bool(t ^ b);
            }
            _ => {
                a = exp_a;
            }
        }
        return a;
    }
    fn eval_binop(&mut self, exp: Expr) -> Expr {
        let stack = self.get_stack();
        {
            let mut stack = stack.lock().unwrap();
            *stack += 1;

            if *stack < RECURS_LIMIT {
                drop(stack);
                match exp {
                    Expr::BinOp(exp_a, op, exp_b) => {
                        let exp_a = *exp_a;
                        let exp_b = *exp_b;

                        let mut new_exp = Expr::New;
                        match op {
                            Token::Add => {
                                new_exp = self.add_exp(exp_a,exp_b);
                            }
                            Token::Sub => {
                                new_exp = self.sub_exp(exp_a,exp_b);
                            }
                            Token::Multiply => {
                                new_exp = self.mult_exp(exp_a,exp_b);
                            }
                            Token::Divide => {
                                new_exp = self.div_exp(exp_a,exp_b);
                            }
                            Token::Mod => {
                                new_exp = self.mod_exp(exp_a,exp_b);
                            }
                            Token::Eq => {
                                new_exp = self.eq_exp(exp_a,exp_b);
                            }
                            Token::Neq => {
                                new_exp = self.neq_exp(exp_a,exp_b);
                            }
                            Token::Lt => {
                                new_exp = self.lt_exp(exp_a,exp_b);
                            }
                            Token::LtEq => {
                                new_exp = self.lteq_exp(exp_a,exp_b);
                            }
                            Token::Gt => {
                                new_exp = self.gt_exp(exp_a,exp_b);
                            }
                            Token::GtEq => {
                                new_exp = self.gteq_exp(exp_a,exp_b);
                            }
                            Token::Or => {
                                new_exp = self.or_exp(exp_a,exp_b);
                            }
                            Token::And => {
                                new_exp = self.and_exp(exp_a,exp_b);
                            }
                            Token::Xor => {
                                new_exp = self.xor_exp(exp_a,exp_b);
                            }
                            _ => {

                            }
                        }
                        let stack = self.get_stack();
                        {
                            let mut stack = stack.lock().unwrap();
                            *stack -= 1;
                        }
                        return new_exp;
                    }
                    _ => {
                    }
                }
            }
        }
        let stack = self.get_stack();
        {
            let mut stack = stack.lock().unwrap();
            *stack -= 1;
        }
        return exp;
        
    }
    fn eval_exp(&mut self, index: Option<String>, name: String) -> Expr {
        let mut expr_a = Expr::New;
        let mut op = Token::Illegal(0,"expected an op".to_string());
        parse!(|self.peek(), token| {
            Token::RParen => {
                self.advance();
                break;
            }

            Token::LParen => {
                self.advance();
                let new_exp = self.eval_exp(index.clone(), name.clone());
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());

            }
            Token::Semicolon => {
                self.advance();
                match expr_a {
                    Expr::New => {
                        expr_a = Expr::Bool(false);
                    }
                    _ => {}
                }
                if let Some(i) = self.vars.get_mut(&name) {
                    i.insert(index, expr_a.clone());
                } else {
                    let mut new_exp: HashMap<Option<String>, Expr> = HashMap::new();
                    new_exp.insert(index, expr_a.clone());
                    self.vars.insert(name, new_exp);
                }
                break;
            }
            Token::Call(name, args) => {
                self.advance();
                self.call_func(name, args);
                let mut new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                {
                    let argreg = self.get_argreg();
                    let mut argreg = argreg.lock().unwrap();
                    // bet you didn't know you could
                    // index None. I feel like this shouldn't
                    // be allowed. I would love to know how the
                    // hashing function handles this. This may
                    // seem like a bad practice, but this
                    // ensures that the user can only access
                    // this register by not indexing in the actual language.
                    let index: Option<String> = None;  
                    
                    if let Some(i) = argreg.get_mut(&index) {
                        new_exp = i.clone();
                    } else {
                        //TODO: serendipity
                    }
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }
            Token::Argreg(index) => {
                self.advance();
                let mut new_exp = Expr::Bool(false);
                {
                    let argreg = self.get_argreg();
                    let mut argreg = argreg.lock().unwrap();
                    let mut index_eval: Option<String> = None;
                    //yes very readable. took two days to fiure out what was going on;
                    if let Some(i) = index {
                        if let Some(i) = self.vars.get(&i){
                            if let Some(i) = i.get(&index_eval) {
                                index_eval = Some(Expr::parse_exp_string(i.clone()));
                            }
                        }
                    }

                    if let Some(i) = argreg.get_mut(&index_eval) {
                        new_exp = i.clone();
                    } else {
                        //TODO: serendipity
                    }
                }
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }
            Token::Identifier(index, name) => {
                self.advance();
                let mut new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if let Some(i) = self.vars.get_mut(&name) {
                    if let Some(i) = i.get(&index) {
                        new_exp = i.clone();
                    }
                } else {
                    //TODO: serendipity
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }
            Token::Integer(val) => {
                self.advance();
                let new_exp = Expr::Int(val);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }

            Token::Float(val) => {
                self.advance();
                let new_exp = Expr::Float(val);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }

            Token::Bool(val) => {
                self.advance();
                let new_exp = Expr::Bool(val);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }

            Token::String(val) => {
                self.advance();
                let new_exp = Expr::String(val);
                //TODO: serendipity
                let mut has_a = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if has_a {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                } else {
                    expr_a = new_exp;
                }
                op = Token::Illegal(0,"expected an op".to_string());
            }

            Token::Multiply => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Multiply;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Divide => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Divide;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Add => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Add;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Sub => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Sub;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }

            Token::Mod => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Mod;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }


            Token::Eq => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Eq;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Neq => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Neq;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }

            Token::Lt => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Lt;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Gt => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Gt;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::LtEq => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::LtEq;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::GtEq => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::GtEq;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::And => {
                self.advance();

                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::And;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Or => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Or;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            Token::Xor => {
                self.advance();
                let new_exp = Expr::Bool(false);
                //TODO: serendipity
                let mut has_a = false;
                let mut has_op = false;
                match expr_a {
                    Expr::New => {}
                    _ => {has_a = true;}
                }
                match op {
                    Token::Illegal(_,_) => {
                        if !has_a {
                            expr_a = new_exp.clone();
                        } else {
                            op = Token::Xor;
                        }
                    }
                    _ => {
                        has_op = true;
                    }
                }
                if has_a && has_op {
                    expr_a = self.eval_binop(Expr::BinOp(Box::new(expr_a.clone()), op.clone(), Box::new(new_exp)));
                    op = Token::Illegal(0,"expected an op".to_string());
                } 
            }
            _ => {
                match expr_a {
                    Expr::New => {
                        expr_a = Expr::Bool(false);
                        //TODO: serendipity
                    }
                    _ => {}
                }
                if let Some(i) = self.vars.get_mut(&name) {
                    i.insert(index, expr_a.clone());
                } else {
                    let mut new_exp: HashMap<Option<String>, Expr> = HashMap::new();
                    new_exp.insert(index, expr_a.clone());
                    self.vars.insert(name, new_exp);
                }
                break;
            }
        });
        return expr_a;
    }

    fn skip_def(&mut self, def_name: String) -> () {
        let mut paren = 1;
        let mut tokens: Vec<Token> = Vec::new();

        parse!(|self.peek(), token| {
            Token::Function(_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::If(_,_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::Loop(_,_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::LBrack => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::RBrack => {
                paren -= 1;
                self.tokens.remove(self.position);
                if paren == 0 {
                    break;
                } else {
                    tokens.push(token);
                }
                
            }
            _ => {
                self.tokens.remove(self.position);
                tokens.push(token);
            }
        });

        let defs = self.get_defs();
        {
            let mut defs = defs.lock().unwrap();
            defs.insert(def_name, tokens.iter().rev().cloned().collect());
        }
    }

    fn skip_fun(&mut self) -> Fun {
        let mut paren = 1;
        let mut tokens: Vec<Token> = Vec::new();

        parse!(|self.peek(), token| {
            Token::Function(_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::If(_,_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::Loop(_,_) => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::LBrack => {
                self.tokens.remove(self.position);
                tokens.push(token);
                paren += 1;
            }
            Token::RBrack => {
                paren -= 1;
                self.tokens.remove(self.position);
                if paren == 0 {
                    break;
                } else {
                    tokens.push(token);
                }
                
            }
            _ => {
                self.tokens.remove(self.position);
                tokens.push(token);
            }
        });

        let fun = Fun::new_sub(tokens,self.get_defs(), 
                               self.get_argreg(), 
                               self.get_funcs(), 
                               self.get_stack(), 
                               self.get_output());
        return fun;
    }

    //TODO: Add errors
    fn output_string(&mut self) -> () {
        parse!(|self.peek(), token| {
            Token::String(to_print) => {
                self.advance();
                
                let printer = self.get_output();
                {
                    let mut printer = printer.lock().unwrap();
                    printer.push_str(&to_print);
                }

                parse!(|self.peek(), token| {
                    Token::Semicolon => {
                        self.advance();
                    }
                    _ => {
                        //TODO: serendipity
                        break;
                    }
                });
            }
            _ => {
                //TODO: serendipity
                break;
            }
        });


    }

    fn peek(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens.get(self.position);
            let token = token.unwrap().clone(); // shouldn't be possible to have none.
            Some(token)
        } else {
            None
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }

    fn get_defs(&self) -> ArcDefs {
        return Arc::clone(&self.defs);
    }

    fn get_argreg(&self) -> ArcArg {
        return Arc::clone(&self.argreg);
    }

    fn get_funcs(&self) -> ArcFuns {
        return Arc::clone(&self.functions);
    
    }

    fn get_stack(&self) -> SoMonitor {
        return Arc::clone(&self.stack);
    }

    pub fn get_output(&self) -> Printer {
        return Arc::clone(&self.output);
    }

    fn call_func(&mut self, fun_name: String, arg_name: String) -> () {
        let mut func_test: Fun = Fun::new(Vec::new());
        let funcs = self.get_funcs();
        {
            let mut funcs = funcs.lock().unwrap();
            if let Some(func) = funcs.get_mut(&fun_name) {
                func_test = func.clone();
                let inner_map = self.vars.get(&arg_name);  // Replace "some_key" with the actual key
                if let Some(inner_map) = inner_map {
                    let cloned_inner_map = inner_map.clone();
                    func_test.vars.insert(String::from("_@"), cloned_inner_map);
                }else{
                    //TODO: serendipity
                }
            }
        } let stack = self.get_stack();
        {
            let mut stack = stack.lock().unwrap();
            *stack += 1;

            if *stack < RECURS_LIMIT {
                drop(stack);
                //println!("{:?}", func_test.tokens);
                func_test.eval();
            } else {
                //TODO: serendipity
            }
        }
    }
    fn skip_loop(&mut self, mut paren: i64) -> () {

        //println!("skipped {}",paren);
        parse!(|self.peek(), token| {
            Token::Function(_) => {
                self.advance();
                paren += 1;
            }
            Token::If(_,_) => {
                self.advance();
                paren += 1;
            }
            Token::Loop(_,_) => {
                self.advance();
                paren += 1;
            }
            Token::LBrack => {
                self.advance();
                paren += 1;
            }
            Token::RBrack => {
                self.advance();
                paren -= 1;
                if paren == 0 {
                    break;
                }                
            }
            _ => {
                self.advance();
            }
        });


    }

    pub fn eval(&mut self) -> () {
        parse!(|self.peek(), token| {
            Token::If(index, name) => {
                if let Some(mut count) = self.paren_counter.pop() {
                    count += 1;
                    self.paren_counter.push(count);
                }else{
                    //println!("if{:?}",self.paren_counter);
                    self.paren_counter.push(1);
                }
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }
                let mut check_var: Option<Expr> = None;
                if let Some(var) = self.vars.get(&name) {
                        if let Some(var) = var.get(&index_eval) {
                            check_var = Some(var.clone());
                        }
                }
                if let Some(check_var) = check_var{
                    //println!("if{:?}",self.paren_counter);
                    if Expr::parse_exp_boolean(check_var) {
                        self.advance();
                        //println!("begin_if{:?}",self.paren_counter);
                    } else {
                        self.skip_loop(0);
                        if let Some(mut count) = self.paren_counter.pop() {
                            count -= 1;
                            self.paren_counter.push(count);
                        }       
                    }
                } else {
                    //TODO: serendipity
                }
            }
            Token::Loop(index, name) => {
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }
                let mut check_var: Option<Expr> = None;
                if let Some(var) = self.vars.get(&name) {
                    if let Some(var) = var.get(&index_eval) {
                        check_var = Some(var.clone());
                    }
                }
                if let Some(check_var) = check_var{
                    if Expr::parse_exp_boolean(check_var) {
                        self.loop_start.push(self.position);
                        self.advance();
                        self.paren_counter.push(1);
                    } else {
                        self.skip_loop(0);
                    }
                //println!("loop{:?}",self.paren_counter);
                } else {
                    //TODO: serendipity
                }
            }
            Token::Break => {
                self.advance();
                if let Some(position) = self.loop_start.pop(){
                    self.position = position;
                }
                self.skip_loop(0);
            }
            Token::RBrack => {
                        //println!("\nright_brack_peek{:?} at {}",self.peek(), position);
                self.advance();
                        //println!("\nright_brack_peek{:?} at {}",self.peek(), position);
                if let Some(mut count) = self.paren_counter.pop() {
                    count -= 1;
                    if count.clone() == 0 {
                        if let Some(position) = self.loop_start.pop(){
                            self.position = position;
                        }
                    } else {
                        //
                        //println!("rb{:?}",count);
                        self.paren_counter.push(count);
                    }
                }else{
                    //if let Some(position) = self.loop_start.pop(){
                        //self.position = position;
                    //}
                }

            }
            Token::Argreg(index) => {
                self.advance();
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }

                let exp = self.eval_exp(index_eval.clone(), String::from("@"));
                let argreg = self.get_argreg();
                {
                    let mut argreg = argreg.lock().unwrap();
                    argreg.insert(index_eval, exp);
                }

            }

            Token::Kill(_, name) => {
                self.advance();
                self.vars.remove(&name);
                self.vars.shrink_to_fit();
            }
            Token::Assign(index, name) => {
                self.advance();
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }
                self.eval_exp(index_eval, name);
            }
            Token::Output(index, name) => {
                let mut to_print = String::new();
                self.advance();
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }
                if let Some(vars) = self.vars.get(&name) {
                    if let Some(output) = vars.get(&index_eval) {
                        match output {
                            Expr::Bool(t) => {
                                to_print.push_str(&t.to_string());
                            }
                            Expr::Int(t) => {
                                to_print.push_str(&t.to_string());
                            }
                            Expr::Float(t) => {
                                to_print.push_str(&t.to_string());
                            }
                            Expr::String(t) => {
                                to_print.push_str(&t.to_string());
                            }
                            _ => {
                                //TODO: serendipity
                            }
                        }
                    } else {
                        //TODO: serendipity
                    }
                } else {
                    //TODO: serendipity
                }
                let printer = self.get_output();
                {
                    let mut printer = printer.lock().unwrap();
                    printer.push_str(&to_print);
                }
                
            }
            Token::OutputStr => {
                self.advance();
                self.output_string();
            }
            Token::Input(index, name) => {
                self.advance();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input = input.trim().to_string();
                //TODO: serendipity
                let mut index_eval: Option<String> = None;
                //yes very readable. took two days to fiure out what was going on;
                if let Some(i) = index {
                    if let Some(i) = self.vars.get(&i){
                        if let Some(i) = i.get(&index_eval) {
                            index_eval = Some(Expr::parse_exp_string(i.clone()));
                        }
                    }
                }
                if let Some(i) = self.vars.get_mut(&name) {
                    i.insert(index_eval, Expr::String(input));
                } else {
                    let mut new_exp: HashMap<Option<String>, Expr> = HashMap::new();
                    new_exp.insert(index_eval, Expr::String(input));
                    self.vars.insert(name, new_exp);
                }
            }
            Token::Call(fun_name, arg_name) => {
                self.advance();
                self.call_func(fun_name, arg_name);
            }
            Token::Def(def_name) => {
                self.tokens.remove(self.position);
                parse!(|self.peek(), token| {
                    Token::LBrack => {
                        self.tokens.remove(self.position);
                        self.skip_def(def_name);
                        break;
                    }
                    _ => {break}

                });
            }
            Token::Insert(def_name) => {
                self.tokens.remove(self.position);
                let defs = self.get_defs();
                {
                    let defs = defs.lock().unwrap();
                    if let Some(def) = defs.get(&def_name) {
                        for token in def {
                            self.tokens.insert(self.position, token.clone());
                        }
                    }
                }
            }
            Token::Function(name) => {
                self.tokens.remove(self.position);
                let fun = self.skip_fun(); 

                let funcs = self.get_funcs();
                {
                    let mut funcs = funcs.lock().unwrap();
                    funcs.insert(name,fun);
                }
            }
            
            _ => {
                //TODO: serendipity
                self.advance();
            }
        });

        //should work like garbage collection, idk
        //should probably add a kill function token
        let stack = self.get_stack();
        {
            let mut stack = stack.lock().unwrap();
            *stack -= 1;
        }
        self.vars.clear();
        self.vars.shrink_to(0);
    }

    fn new_sub(tokens: Vec<Token>, defs: ArcDefs, argreg: ArcArg, functions: ArcFuns, somon: SoMonitor, output: Printer) -> Fun {
        Fun{
            vars: HashMap::new(),
            defs: defs,
            argreg: argreg,
            functions: functions,
            tokens: tokens,
            position: 0,
            loop_start: Vec::new(),
            paren_counter: Vec::new(),
            stack: somon,
            output: output,
        }
        
    }
     
    pub fn new(tokens: Vec<Token>) -> Fun{
        Fun{
            vars: HashMap::new(),
            defs: Arc::new(Mutex::new(HashMap::new())),
            argreg: Arc::new(Mutex::new(HashMap::new())),
            functions: Arc::new(Mutex::new(HashMap::new())),
            tokens: tokens,
            position: 0,
            loop_start: Vec::new(),
            paren_counter: Vec::new(),
            stack: Arc::new(Mutex::new(1)),
            output: Arc::new(Mutex::new(String::new())),
        }
    }
}
