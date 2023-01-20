use parser;
use typing;
use eval::{*,ValEnvStack};



/*
pub struct GC{
    pub(crate) closures : Vec<Closure>,
}

impl GC{
    pub fn new() -> GC{
        GC{closures:Vec::new()}
    }
    pub fn add(&mut self, c : Closure){
        self.closures.push(c);
    }
    pub fn get(&self, key : &str) -> Option<parser::Expr>{
        for c in self.closures.iter(){
            if let Some(e) = c.get(key){
                return Some(e);
            }
        }
        None
    }
}*/

