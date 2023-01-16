# Linzin

Linzin is a linear type system dirived from [Linz](https://github.com/ytakano/rust_zero/tree/master/ch09/linz).


「ゼロから学ぶRust」で題材とされたLinzというλ計算に線形型システムを適用した独自の線形型言語の拡張実装です.

## TODO
- [x] //で一行コメント機能
- [x] アフィン型追加
- [x] 評価器の実装
- [ ] ガベージコレクションの実装(マークアンドスイープ)

## Linzinの構文
```text
<VAR>   := 1文字以上のアルファベットから成り立つ変数

<E>     := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL>
<LET>   := let <VAR> : <T> = <E>; <E>
<IF>    := if <E> { <E> } else { <E> }
<SPLIT> := split <E> as <VAR>, <VAR> { <E> }
<FREE>  := free <E>; <E>
<APP>   := ( <E> <E> )

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
$ cargo run codes/ex12.lin
```
- 出力例
```
[src/main.rs:55] &new = "let z : lin (lin (lin bool * lin bool) -> lin bool) = lin fn x : lin (lin bool * lin bool) {    split x as a, b {        if a {            b        } else {            b        }    }};(z  lin <lin true, lin false>)"
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


---
## Ref.
- ゼロから学ぶRust システムプログラミングの基礎から線形型システム, 高野祐輝, 講談社
- https://github.com/ytakano/rust_zero