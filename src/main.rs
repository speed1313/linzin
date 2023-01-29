mod eval;
mod helper;
mod parser;
mod typing;
mod gc;
use nom::error::convert_error;
use nom::{error::VerboseError, IResult};
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
        println!(
            "Welcome to Linzin!\nLet's type <expression>\nTo show the environment, please type env"
        );
        loop {
            // 1行読み込んでパースし成功すれば評価
            if let Ok(readline) = rl.readline(">> ") {
                let content = skip_comment(&readline); // コメントを削除
                if content.eq("env") {
                    println!("[Type Environment]:\n {:?}", ctx);
                    println!("[Variable Environment]\n {:?}", val_env);
                    continue;
                }
                let ast = parser::parse(&content); // パース

                interpret(&content, &mut ctx, &mut val_env, ast);
            } else {
                break;
            }
        }
        return Ok(());
    }

    // ファイル読み込み
    let content = fs::read_to_string(&args[1])?;
    let content = skip_comment(&content); // コメントを削除
    let ast = parser::parse(&content); // パース
    println!("AST:\n{:#?}\n", ast);
    let mut ctx = typing::TypeEnv::new();
    let mut val_env = eval::ValEnv::new();
    interpret(&content, &mut ctx, &mut val_env, ast);
    Ok(())
}

fn interpret(
    content: &str,
    ctx: &mut typing::TypeEnv,
    val_env: &mut eval::ValEnv,
    ast: IResult<&str, parser::Expr, VerboseError<&str>>,
) {
    match ast {
        Ok((_, expr)) => {
            // println!("[Expression]\n{content}");
            // typing
            let ty = match typing::typing(&expr, ctx, 0) {
                Ok(a) => a,
                Err(e) => {
                    println!("typing error:\n{e}");
                    return;
                }
            };
            println!("[Type]\n{}", ty);

            // evaluation
            let result = match eval::eval(&expr, ctx, val_env, 0) {
                Ok(v) => v,
                Err(e) => {
                    println!("evaluation error:\n{e}");
                    return;
                }
            };
            println!("[Evaluation]\n{}", result);
        }
        Err(nom::Err::Error(e)) => {
            let msg = convert_error(content, e);
            eprintln!("parse error:\n{msg}");
        }
        _ => (),
    }
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
