use crate::Token;
//TODO seperate exp functions from Fun to Expr impl
#[derive(Debug, Clone)]
pub enum Expr {
    New,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    BinOp(Box<Expr>, Token, Box<Expr>),
}
impl Expr {
    pub fn parse_exp_string(exp: Expr) -> String {
        let mut string: String = String::from("tom is a genius");
        match exp{
            Expr::Bool(t) => {
                if t {
                    //python bool formating sucks
                    //just had to comment on that
                    //not like this will be any better 
                    string = String::from("true");
                } else {
                    string = String::from("false");
                }
            }
            Expr::Int(t) => {
                string = t.to_string();
            }
            Expr::Float(t) => {
                
                string = t.to_string();
            }
            Expr::String(t) => {
                return t;
            }
            _ => {
            }
        }
        return string;
    }
    pub fn parse_exp_float(exp: Expr) -> f64 {
        let mut float: f64 = 420.0;
        match exp{
            Expr::Bool(t) => {
                if t {
                    float = 1.0;
                } else {
                    float = 0.0;
                }
            }
            Expr::Int(t) => {
                float = t as f64;
            }
            Expr::Float(t) => {
                float = t;
            }
            Expr::String(t) => {
                if let Ok(float_untested) = t.parse::<f64>(){
                    float = float_untested;
                }
            }
            _ => {
            }
        }
        return float;
    }
    pub fn parse_exp_integer(exp: Expr) -> i64 {
        let mut integer: i64 = 69;
        match exp{
            Expr::Bool(t) => {
                if t {
                    integer = 1;
                } else {
                    integer = 0;
                }
            }
            Expr::Int(t) => {
                integer = t;
            }
            Expr::Float(t) => {
                integer = t as i64;
            }
            Expr::String(t) => {
                integer = t.len() as i64;
            }
            _ => {
            }
        }
        return integer;
    }
    
    pub fn parse_exp_boolean(exp: Expr) -> bool {
        let mut boolean: bool = false;
        match exp{
            Expr::Bool(t) => {
                boolean = t;
            }
            Expr::Int(t) => {
                if t > 0 {
                    boolean = true;
                } else {
                    boolean = false;
                }
            }
            Expr::Float(t) => {
                if t > 0.0 {
                    boolean = true;
                } else {
                    boolean = false;
                }
            }
            Expr::String(t) => {
                if t.len()==0 {
                    boolean = false;
                } else {
                    boolean = true;
                }
            }
            _ => {
            }
        }
        return boolean;
    }

}
