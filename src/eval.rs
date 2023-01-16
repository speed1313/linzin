//! 線形型システムの評価器
//!
//!
//! typingと同様にして値を評価する.
//! Eの部分は評価したらboolになる?のでそれを評価して分岐
//! とりあえずtupleはむし
//! 将来的には<VAL>を返し, 受け取り, matchで分岐処理する

use crate::{helper::safe_add, parser, typing};
use std::{borrow::Cow, collections::BTreeMap};

type VarToVal = BTreeMap<String, Option<ReturnVal>>;

type VResult<'a> = Result<ReturnVal, Cow<'a, str>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReturnVal {
    Bool(bool),          // 真偽値リテラル
    Pair(bool, bool),    // ペア
    Fun(parser::FnExpr), // 関数
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValEnv {
    env: ValEnvStack,
}

impl ValEnv {
    pub fn new() -> ValEnv {
        ValEnv {
            env: ValEnvStack::new(),
        }
    }

    /// 変数環境をpush
    fn push(&mut self, depth: usize) {
        self.env.push(depth);
    }

    /// 変数環境をpop
    fn pop(&mut self, depth: usize) -> Option<VarToVal> {
        let v = self.env.pop(depth);
        v
    }

    /// 変数環境へ変数と値をpush
    fn insert(&mut self, key: String, value: ReturnVal) {
        self.env.insert(key, value);
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut Option<ReturnVal>> {
        if let Some((_, t)) = self.env.get_mut(key) {
            Some(t)
        } else {
            unreachable!()
        }
    }
}

/// 変数環境のスタック
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct ValEnvStack {
    vars: BTreeMap<usize, VarToVal>,
}

impl ValEnvStack {
    fn new() -> ValEnvStack {
        ValEnvStack {
            vars: BTreeMap::new(),
        }
    }

    // 変数環境をpush
    fn push(&mut self, depth: usize) {
        self.vars.insert(depth, BTreeMap::new());
    }

    // 変数環境をpop
    fn pop(&mut self, depth: usize) -> Option<VarToVal> {
        self.vars.remove(&depth)
    }

    // スタックの最も上にある変数環境に変数名と値を追加
    fn insert(&mut self, key: String, value: ReturnVal) {
        if let Some(last) = self.vars.iter_mut().next_back() {
            last.1.insert(key, Some(value));
        }
    }

    // スタックを上から辿っていき, 初めに見つかる変数の値を取得
    fn get_mut(&mut self, key: &str) -> Option<(usize, &mut Option<ReturnVal>)> {
        for (depth, elm) in self.vars.iter_mut().rev() {
            if let Some(e) = elm.get_mut(key) {
                return Some((*depth, e));
            }
        }
        None
    }
}

pub fn eval<'a>(
    expr: &parser::Expr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    match expr {
        parser::Expr::App(e) => eval_app(e, type_env, val_env, depth),
        parser::Expr::QVal(e) => eval_qval(e, type_env, val_env, depth),
        parser::Expr::Free(e) => eval_free(e, type_env, val_env, depth),
        parser::Expr::If(e) => eval_if(e, type_env, val_env, depth),
        parser::Expr::Split(e) => eval_split(e, type_env, val_env, depth),
        parser::Expr::Var(e) => eval_var(e, type_env, val_env),
        parser::Expr::Let(e) => eval_let(e, type_env, val_env, depth),
    }
}

fn eval_app<'a>(
    expr: &parser::AppExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let arg = eval(&expr.expr2, type_env, val_env, depth)?;
    match &*expr.expr1 {
        parser::Expr::QVal(val) => match val {
            parser::QValExpr {
                qual: _,
                val: parser::ValExpr::Fun(f),
            } => {
                let mut depth = depth;
                safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;
                val_env.push(depth);
                val_env.insert(f.var.clone(), arg);
                let ret = eval(&f.expr, type_env, val_env, depth);
                val_env.pop(depth);
                ret
            }
            _ => Err("function should be applied to a function".into()),
        },
        parser::Expr::Var(a) => {
            if let Some(v) = val_env.clone().get_mut(a).unwrap() {
                match v {
                    ReturnVal::Fun(f) => {
                        let mut depth = depth;
                        safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;
                        val_env.push(depth);

                        val_env.insert(f.var.clone(), arg.clone());
                        let e = eval(&f.expr, type_env, val_env, depth);
                        val_env.pop(depth);
                        e
                    }
                    _ => return Err("function should be applied to a function".into()),
                }
            } else {
                Err("function should be applied to a function".into())
            }
        }
        _ => Err("function should be applied to a function".into()),
    }
}
fn eval_qval<'a>(
    expr: &parser::QValExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let p = match &expr.val {
        parser::ValExpr::Bool(v) => Ok(ReturnVal::Bool(*v)),
        parser::ValExpr::Pair(e1, e2) => {
            let v1 = eval(e1, type_env, val_env, depth)?;
            let v2 = eval(e2, type_env, val_env, depth)?;
            match (v1, v2) {
                (ReturnVal::Bool(v1), ReturnVal::Bool(v2)) => Ok(ReturnVal::Pair(v1, v2)),
                _ => Err("pair values should be bool".into()),
            }
        }
        // 使用する時までASTを保持しておく
        parser::ValExpr::Fun(e) => Ok(ReturnVal::Fun(e.clone())),
    };
    p
}
fn eval_free<'a>(
    expr: &parser::FreeExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    if let Some(a) = val_env.get_mut(&expr.var) {
        *a = None;
        eval(&expr.expr, type_env, val_env, depth)
    } else {
        Err("no variable to free".into())
    }
}
fn eval_if<'a>(
    expr: &parser::IfExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let e1 = match eval(&expr.cond_expr, type_env, val_env, depth) {
        Ok(ReturnVal::Bool(v)) => v,
        _ => panic!("if文の条件式はbool型でなければなりません"),
    };
    if e1 {
        eval(&expr.then_expr, type_env, val_env, depth)
    } else {
        eval(&expr.else_expr, type_env, val_env, depth)
    }
}

fn eval_split<'a>(
    expr: &parser::SplitExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let e = eval(&expr.expr, type_env, val_env, depth)?;
    let mut depth = depth;
    safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;
    match e {
        ReturnVal::Pair(v1, v2) => {
            val_env.push(depth);
            val_env.insert(expr.left.clone(), ReturnVal::Bool(v1));
            val_env.insert(expr.right.clone(), ReturnVal::Bool(v2));
        }
        _ => panic!("splitの引数はpair型でなければなりません"),
    }
    let ret = eval(&expr.body, type_env, val_env, depth);
    let _ = val_env.pop(depth);

    ret
}
fn eval_var<'a>(expr: &str, type_env: &mut typing::TypeEnv, val_env: &mut ValEnv) -> VResult<'a> {
    let val = val_env.get_mut(expr);
    let val = match val {
        Some(v) => v,
        None => return Err("変数が見つかりません".into()),
    };
    let ret = val.clone();
    // もし変数がlinなら, 使用後freeする.
    match type_env.env_lin.get_mut(expr) {
        Some(_) => {
            *val = None;
        }
        None => (),
    }
    match type_env.env_aff.get_mut(expr){
        Some(_) => *val = None,
        None => (),
    }
    ret.ok_or("変数が見つかりません".into())
}

fn eval_let<'a>(
    expr: &parser::LetExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let v1 = match eval(&expr.expr1, type_env, val_env, depth) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let mut depth = depth;
    safe_add(&mut depth, &1, || "変数のスコープのネストが深すぎる").unwrap();
    val_env.push(depth);
    val_env.insert(expr.var.clone(), v1);

    let v2 = eval(&expr.expr2, type_env, val_env, depth);

    _ = val_env.pop(depth);
    v2
}

#[cfg(test)]
mod tests {
    use crate::eval::*;
    use crate::{
        parser,
        parser::{Expr::*, *},
    };

    #[test]
    fn test_eval_var() {
        let expr = QVal(QValExpr {
            qual: Qual::Un,
            val: ValExpr::Bool(true),
        });
        let result = match eval(&expr, &mut typing::TypeEnv::new(), &mut ValEnv::new(), 0) {
            Ok(ReturnVal::Bool(v)) => v,
            _ => panic!("eval_varのテストでエラーが発生しました"),
        };
        assert_eq!(true, result);
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
        if let Ok((_, expr)) = parser::parse_expr(input) {
            let result = match eval(&expr, &mut typing::TypeEnv::new(), &mut ValEnv::new(), 0) {
                Ok(ReturnVal::Bool(v)) => v,
                _ => panic!("eval_varのテストでエラーが発生しました"),
            };
            assert_eq!(false, result);
            return;
        }
        unreachable!();
    }
}
