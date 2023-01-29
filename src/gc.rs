use crate::eval::{ValEnvStack, *};
use crate::parser::FnExpr;
use crate::{eval, parser, typing};

// クロージャが持つ環境から, 不要な変数を削除する

pub struct Object {
    closure: Closure,
    is_marked: bool,
}
pub struct GC {
    pub(crate) closures: Vec<Object>,
}

impl GC {
    pub fn new() -> GC {
        GC {
            closures: Vec::new(),
        }
    }
    pub fn insert(&mut self, c: Closure) {
        self.closures.push(Object {
            closure: c,
            is_marked: false,
        });
    }

    pub fn mark(&mut self, v: ValEnvStack, f: FnExpr) {
        // closure内の変数を探索
    }
}
