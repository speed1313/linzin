//! 線形型システムの評価器
//!
//!
//! typingと同様にして値を評価する.
//! Eの部分は評価したらboolになる?のでそれを評価して分岐
//! とりあえずtupleはむし
use crate::{helper::safe_add, parser,typing};
use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap, mem};

type VarToVal = BTreeMap<String, Option<bool>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValEnv {
    env: ValEnvStack,
}

impl ValEnv{
    pub fn new() -> ValEnv {
        ValEnv { env: ValEnvStack::new() }
    }

    /// 変数環境をpush
    fn push(&mut self, depth: usize){
        self.env.push(depth);
    }

    /// 変数環境をpop
    fn pop(&mut self, depth: usize)-> Option<VarToVal>{
        let v = self.env.pop(depth);
        v
    }

    /// 変数環境へ変数と値をpush
    fn insert(&mut self, key: String, value: bool) {
        self.env.insert(key, value);
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut Option<bool>> {
        if let Some((d,t)) = self.env.get_mut(key){
            Some(t)
        }else{
            unreachable!()
        }
    }

}

/// 変数環境のスタック
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct ValEnvStack{
    vars: BTreeMap<usize, VarToVal>,
}

impl ValEnvStack{
    fn new() -> ValEnvStack{
        ValEnvStack { vars: BTreeMap::new() }
    }

    // 変数環境をpush
    fn push(&mut self, depth: usize){
        self.vars.insert(depth, BTreeMap::new());
    }

    // 変数環境をpop
    fn pop(&mut self, depth: usize)-> Option<VarToVal>{
        self.vars.remove(&depth)
    }

    // スタックの最も上にある変数環境に変数名と値を追加
    fn insert(&mut self, key: String, value: bool){
        if let Some(last) = self.vars.iter_mut().next_back(){
            last.1.insert(key, Some(value));
        }
    }

    // スタックを上から辿っていき, 初めに見つかる変数の値を取得
    fn get_mut(&mut self, key: &str) -> Option<(usize, &mut Option<bool>)>{
        for (depth, elm) in self.vars.iter_mut().rev(){
            if let Some(e) = elm.get_mut(key){
                return Some((*depth, e));
            }
        }
        None
    }
}



pub fn eval(expr: &parser::Expr, type_env: &mut typing::TypeEnv, val_env: &mut ValEnv, depth: usize) -> bool{
    match expr{
        parser::Expr::App(e) => eval_app(e, type_env,val_env, depth),
        parser::Expr::QVal(e) => eval_qval(e, type_env, val_env,depth),
        parser::Expr::Free(e) => eval_free(e, type_env, val_env, depth),
        parser::Expr::If(e) => eval_if(e, type_env,val_env, depth),
        parser::Expr::Split(e) => eval_split(e, type_env, val_env,depth),
        parser::Expr::Var(e) => eval_var(e, type_env, val_env),
        parser::Expr::Let(e) => eval_let(e, type_env,val_env, depth),
    }
}

fn eval_app(expr: &parser::AppExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    todo!();
}
fn eval_qval(expr: &parser::QValExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    let p = match &expr.val{
        parser::ValExpr::Bool(v) => v,
        parser::ValExpr::Pair(e1,e2) =>{
            let v1 = eval(e1, type_env, val_env, depth);
            let v2 = eval(e2, type_env, val_env, depth);
            todo!("return pair value");
        },
        parser::ValExpr::Fun(e) => {
            todo!("return function value");
        },
    };
    *p
}
fn eval_free(expr: &parser::FreeExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    todo!();
}
fn eval_if(expr: &parser::IfExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    todo!();
}

fn eval_split(expr: &parser::SplitExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    todo!();
}
fn eval_var(expr: &str, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv)->bool{

    todo!();
}
fn eval_let(expr: &parser::LetExpr, type_env: &mut typing::TypeEnv,val_env: &mut ValEnv, depth:usize)->bool{
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::{parser,parser::{Expr::*, *}};
    use crate::{eval::*, typing::*};


    #[test]
    fn test_eval_qval() {
        let expr = QVal(
            QValExpr {
                qual: Qual::Un,
                val: ValExpr::Bool(true),
            });
        assert_eq!(true, eval(&expr, &mut typing::TypeEnv::new(), &mut ValEnv::new(), 0));
    }
    #[test]
    fn test_eval_if() {
        let input = r"let x : un bool = un true;
            if x {
                un false
            } else {
                un true
            }
            ";
        if let Ok((_,expr) )= parser::parse_expr(input){
            assert_eq!(false, eval(&expr, &mut typing::TypeEnv::new(), &mut ValEnv::new(), 0));
            return;
        }
        unreachable!();


    }
}