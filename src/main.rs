mod eval;
mod helper;
mod parser;
mod typing;

use nom::error::convert_error;
use rustyline::Editor;
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数の検査
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // eprintln!("以下のようにファイル名を指定して実行してください\ncargo run codes/ex1.lin");
        // return Err("引数が不足".into());
        let mut rl = Editor::<()>::new().unwrap();
        let mut ctx = typing::TypeEnv::new();
        let mut val_env = eval::ValEnv::new();
        // prepare global environment
        ctx.push(0);
        val_env.push(0);
        println!("Welcome to Linzin!\nLet's type <expression>\nTo show the environment, please type env");
        loop {
            // 1行読み込んでパースし成功すれば評価
            if let Ok(readline) = rl.readline(">> ") {
                let content = skip_comment(&readline); // コメントを削除
                if content.eq("env"){
                    println!("type env:\n {:?}", ctx);
                    println!("val env:\n {:?}", val_env);
                    continue;
                }
                let ast = parser::parse_expr(&content); // パース
                match ast {
                    Ok((_, expr)) => {
                        println!("式:\n{content}");
                        // 型付け
                        match typing::typing(&expr, &mut ctx, 0) {
                            Ok(a) => println!("の型は\n{a}\nです。"),
                            Err(e) => {
                                println!("型付けエラー:\n{e}");
                                continue;
                            }
                        }
                        // println!("ctx: {:?}", ctx);
                        let result;
                        match eval::eval(&expr, &mut ctx, &mut val_env, 0) {
                            Ok(v) => result = v,
                            Err(e) => {
                                println!("評価エラー:\n{e}");
                                continue;
                            }
                        }
                        // println!("val_env: {:?}", val_env);
                        println!("評価結果: {:?}", result);
                    }
                    Err(nom::Err::Error(e)) => {
                        let msg = convert_error(content.as_str(), e);
                        eprintln!("パースエラー:\n{msg}");
                    }
                    _ => (),
                }
            } else {
                break;
            }
        }
        return Ok(());
    }

    // ファイル読み込み
    let content = fs::read_to_string(&args[1])?;
    let content = skip_comment(&content); // コメントを削除
    let ast = parser::parse_expr(&content); // パース
    println!("AST:\n{:#?}\n", ast);
    match ast {
        Ok((_, expr)) => {
            let mut ctx = typing::TypeEnv::new();
            println!("式:\n{content}");

            // 型付け
            let a = typing::typing(&expr, &mut ctx, 0)?;
            println!("の型は\n{a}\nです。");
            let val_env = &mut eval::ValEnv::new();
            println!("result: {:?}", eval::eval(&expr, &mut ctx, val_env, 0)?);
        }
        Err(nom::Err::Error(e)) => {
            let msg = convert_error(content.as_str(), e);
            eprintln!("パースエラー:\n{msg}");
            return Err(msg.into());
        }
        _ => (),
    }

    Ok(())
}

// strip comment from input
fn skip_comment(input: &str) -> String {
    let mut new = String::new();
    for i in input.lines() {
        match i.find("//") {
            Some(start) => new.push_str(&i[..start]),
            None => new.push_str(i),
        }
    }
    new
}
