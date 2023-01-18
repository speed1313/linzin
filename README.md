# Linzin

Linzin is a linear type system dirived from [Linz](https://github.com/ytakano/rust_zero/tree/master/ch09/linz).


「ゼロから学ぶRust」で題材とされたLinzというλ計算に線形型システムを適用した独自の線形型言語の拡張実装です.

## TODO
- [x] //で一行コメント機能
- [x] アフィン型追加
- [x] 評価器の実装
- [ ] ガベージコレクションの実装(マークアンドスイープ)
   - un型はfreeが使えないようにしている. それをガベコレする(REPLでない場合, un型が使われないとわかった瞬間にfreeすれば良さそう?)
   - heap型を定義して, heap型はglobal変数みたいに扱ってガベコレ?
- [x] replで実行できるようにする

## Linzinの構文
```text
<VAR>   := 1文字以上のアルファベットから成り立つ変数

<E>     := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL> | <DEF>
<LET>   := let <VAR> : <T> = <E>; <E>
<IF>    := if <E> { <E> } else { <E> }
<SPLIT> := split <E> as <VAR>, <VAR> { <E> }
<FREE>  := free <E>; <E>
<APP>   := ( <E> <E> )
<DEF>   := def <VAR> : <T> = <E>; (REPL専用)

<Q>     := lin | un | aff
```
- 値
```text
<QVAL>  := <Q> <VAL>
<VAL>   := <B> | <PAIR> | <FN>
<B>     := true | false
<PAIR>  := < <E> , <E> >
<FN>    := fn <VAR> : <T> { <E> }
```
- 型
```text
<T>     := <Q> <P>
<P>     := bool |
           ( <T> * <T> )
           ( <T> -> <T> )
```
## How to use
```
$ git clone https://github.com/speed1313/linzin.git
$ cd linzin
$ cat codes/ex12.lin
let z : lin (lin (lin bool * lin bool) -> lin bool) = lin fn x : lin (lin bool * lin bool) {
    split x as a, b {
        if a {
            b
        } else {
            b
        }
    }
};
(z  lin <lin true, lin false>)

$ cargo run codes/ex12.lin
```
- 出力例
```
AST:
...
...

式:
let z : lin (lin (lin bool * lin bool) -> lin bool) = lin fn x : lin (lin bool * lin bool) {    split x as a, b {        if a {            b        } else {            b        }    }};(z  lin <lin true, lin false>)
の型は
lin bool
です。
result: Bool(false)
```

### REPLで遊ぶ
REPLでは, def構文でglobal変数を定義できるようにしています.
```
$ cargo run
Welcome to Linzin!
Let's type <expression>
To show the environment, please type env
>> def x : lin bool = lin true;
式:
def x : lin bool = lin true;
の型は
lin bool
です。
評価結果: Bool(true)

>> (lin fn x : lin bool {
    if x {
        un <un true, un false>
    } else {
        un <un false, un true>
    }
} x)
式:
(lin fn x : lin bool {    if x {        un <un true, un false>    } else {        un <un false, un true>    }} x)
の型は
un (un bool * un bool)
です。
評価結果: Pair(true, false)

>> env
type env:
 TypeEnv { env_lin: TypeEnvStack { vars: {0: {"x": None}} }, env_un: TypeEnvStack { vars: {0: {}} }, env_aff: TypeEnvStack { vars: {0: {}} } }
val env:
 ValEnv { env: ValEnvStack { vars: {0: {"x": None}} } }

>> x
式:
x
型付けエラー:
"x"という変数は定義されていないか、利用済みか、キャプチャできない

```

## Ref.
- ゼロから学ぶRust システムプログラミングの基礎から線形型システム, 高野祐輝, 講談社
- https://github.com/ytakano/rust_zero