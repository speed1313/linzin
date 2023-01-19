//! 線形型システムの評価器
//!
//!

use crate::{helper::safe_add, parser, typing};
use std::{borrow::Cow, collections::BTreeMap, fmt};

type VarToVal = BTreeMap<String, Option<ReturnVal>>;

type VResult<'a> = Result<ReturnVal, Cow<'a, str>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReturnVal {
    Bool(bool),          // 真偽値リテラル
    Pair(bool, bool),    // ペア
    Fun(parser::FnExpr), // 関数
}

impl fmt::Display for ReturnVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnVal::Bool(v) => write!(f, "{v}"),
            ReturnVal::Pair(t1, t2) => write!(f, "({t1} , {t2})"),
            ReturnVal::Fun(expr) => write!(f, "{:?}",expr),
        }
    }
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
    pub fn push(&mut self, depth: usize) {
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
            None
        }
    }
    fn remove(&mut self, key: &str) -> Option<Option<ReturnVal>> {
        if let Some((_, t)) = self.env.remove(key) {
            Some(t)
        } else {
            None
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

    // スタックを上から辿っていき, 初めに見つかる変数の値をremove
    fn remove(&mut self, key: &str) -> Option<(usize, Option<ReturnVal>)> {
        for (depth, elm) in self.vars.iter_mut().rev() {
            if let Some(a) = elm.remove(key){
                return Some((*depth, a));
            }else{
                continue;
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
        parser::Expr::Def(e) => eval_def(e, type_env, val_env, depth),
        parser::Expr::Env(e) => eval_env(e, type_env, val_env, depth),
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
                safe_add(&mut depth, &1, || "Variable scope nesting is too deep")?;
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
                        safe_add(&mut depth, &1, || "Variable scope nesting is too deep")?;
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
        _ => panic!("Conditional expression in if statements must be of type bool"),
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
    safe_add(&mut depth, &1, || "Variable scope nesting is too deep")?;
    match e {
        ReturnVal::Pair(v1, v2) => {
            val_env.push(depth);
            val_env.insert(expr.left.clone(), ReturnVal::Bool(v1));
            val_env.insert(expr.right.clone(), ReturnVal::Bool(v2));
        }
        _ => panic!("The argument of split must be of type pair"),
    }
    let ret = eval(&expr.body, type_env, val_env, depth);
    let _ = val_env.pop(depth);

    ret
}
fn eval_var<'a>(expr: &str, type_env: &mut typing::TypeEnv, val_env: &mut ValEnv) -> VResult<'a> {
    let mut binding = val_env.clone();
    let val = binding.get_mut(expr);
    let val = match val {
        Some(v) => v,
        None => return Err("variable not found".into()),
    };
    let ret = val.clone();
    // もし変数がlinなら, 使用後freeする.
    match type_env.env_lin.get_mut(expr){
        Some(_) => {
            let _ = val_env.remove(expr);
        }
        None => (),
    }
    match type_env.env_aff.get_mut(expr) {
        Some(_) => {
            let _ = val_env.remove(expr);
        }
        None => (),
    }
    ret.ok_or("variable not found".into())
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
    safe_add(&mut depth, &1, || "Variable scope nesting is too deep").unwrap();
    val_env.push(depth);
    val_env.insert(expr.var.clone(), v1);

    let v2 = eval(&expr.expr2, type_env, val_env, depth);
    _ = val_env.pop(depth);

    v2
}

fn eval_def<'a>(
    expr: &parser::DefExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    let v1 = match eval(&expr.expr, type_env, val_env, depth) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    val_env.insert(expr.var.clone(), v1.clone());
    Ok(v1)
}

fn eval_env<'a>(
    expr: &parser::EnvExpr,
    type_env: &mut typing::TypeEnv,
    val_env: &mut ValEnv,
    depth: usize,
) -> VResult<'a> {
    println!("[Type Environment]:\n {:?}", type_env);
    println!("[Variable Environment]\n {:?}", val_env);
    let v = match eval(&expr.expr, type_env, val_env, depth) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(v)
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
            _ => panic!("error happend in eval_var test"),
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
                _ => panic!("error happend in eval_if test"),
            };
            assert_eq!(false, result);
            return;
        }
        unreachable!();
    }
    #[test]
    fn test_eval_app() {
        let input = r"let z : lin (lin (lin bool * lin bool) -> lin bool) = lin fn x : lin (lin bool * lin bool) {
            split x as a, b {
                if a {
                    b
                } else {
                    b
                }
            }
        };
        (z  lin <lin true, lin false>)";
        if let Ok((_, expr)) = parser::parse_expr(input) {
            let result = match eval(&expr, &mut typing::TypeEnv::new(), &mut ValEnv::new(), 0) {
                Ok(ReturnVal::Bool(v)) => v,
                _ => panic!("error happend in eval_app test"),
            };
            assert_eq!(false, result);
            return;
        }
        unreachable!();
    }
}
