# Linzin

Linzin is a linear type system dirived from [Linz](https://github.com/ytakano/rust_zero/tree/master/ch09/linz).

Linzin is an extended implementation of Linz, a linear type system programming language which is the subject of "ゼロから学ぶRust".

## TODO
- [x] one line comment with // feature
- [x] add affine type
- [x] implement interpreter, or evaluator
- [ ] implement garbage collection(mark and sweep)
  - how to do that?
     - the memory of un type can not be free, so Linzin should free the memory automatically?
     - defining heap type and set it to free by gc?
- [x] let the interpreter to be used in REPL format

## Syntax of Linzin
```text
<VAR>   := (alphabet)+ // Variables consisting of one or more letters of the alphabet

<E>     := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL> | <DEF>
<LET>   := let <VAR> : <T> = <E>; <E>
<IF>    := if <E> { <E> } else { <E> }
<SPLIT> := split <E> as <VAR>, <VAR> { <E> }
<FREE>  := free <E>; <E>
<APP>   := ( <E> <E> )
<DEF>   := def <VAR> : <T> = <E>; (for REPL use only)

<Q>     := lin | un | aff
```
- Value
```text
<QVAL>  := <Q> <VAL>
<VAL>   := <B> | <PAIR> | <FN>
<B>     := true | false
<PAIR>  := < <E> , <E> >
<FN>    := fn <VAR> : <T> { <E> }
```
- Type
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
- Output Example
```
AST:
...
...

[Type]
TypeExpr { qual: Lin, prim: Bool }
[Evaluation]
Bool(false)
```

### Playing Linzin in REPL
When you play Linzin in REPL, global variables can be defined with the def syntax.
```
$ cargo run
Welcome to Linzin!
Let's type <expression>
To show the environment, please type env
>> def x : lin bool = lin true;
[Type]
TypeExpr { qual: Lin, prim: Bool }
[Evaluation]
Bool(true)
>> (lin fn x : lin bool {
    if x {
        un <un true, un false>
    } else {
        un <un false, un true>
    }
} x)
[Type]
TypeExpr { qual: Un, prim: Pair(TypeExpr { qual: Un, prim: Bool }, TypeExpr { qual: Un, prim: Bool }) }
[Evaluation]
Pair(true, false)
>> env
[Type Environment]:
 TypeEnv { env_lin: TypeEnvStack { vars: {0: {"x": None}} }, env_un: TypeEnvStack { vars: {0: {}} }, env_aff: TypeEnvStack { vars: {0: {}} } }
[Variable Environment]
 ValEnv { env: ValEnvStack { vars: {0: {"x": None}} } }
>> x
typing error:
The variable "x" is either not defined, already used, or cannot be captured.
```

## Ref.
- ゼロから学ぶRust システムプログラミングの基礎から線形型システム, 高野祐輝, 講談社
- https://github.com/ytakano/rust_zero