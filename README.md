# Linzin

Linzin is a linear type system dirived from [Linz](https://github.com/ytakano/rust_zero/tree/master/ch09/linz).

Linzin is an extended implementation of Linz, a linear type system programming language which is the subject of "ゼロから学ぶRust".

## TODO
- [x] one line comment with // feature
- [x] add affine type
- [x] implement interpreter, or evaluator
- [ ] implement garbage collection(mark and sweep)
  - when the below codes are executed(though Linzin does not support integer), tmp will never be used, but it's memory will have been allocated for a long time.
     ```
        let counter = fn(x){
            if (x > 100){
                return true;
            }else{
                let tmp = 1;
                counter(x + 1);
            }
        };
        counter(0);
     ```
     To avoid this problem, we need to implement garbage collection.
   - By looking forward the structure of the program(AST), we can know which variables will be used in the future, and vice versa.
   - The example of the Linzin program that need gc?
   ```
    let w : un bool = un false;
    let x : un bool = un true;
    let y : un bool = x;
    let z : un bool = un true;
    z
   ```
   Though only z is used in the program, the memory of w,x, and y will be allocated in evaluating for a long time.
   To resolve this problem, before evaluating the expression, we should...?
- [x] let the interpreter to be used in REPL format

## Syntax of Linzin
```text
<VAR>   := (alphabet)+ // Variables consisting of one or more letters of the alphabet

<E>     := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL> | <DEF> | <ENV>
<LET>   := let <VAR> : <T> = <E>; <E>
<IF>    := if <E> { <E> } else { <E> }
<SPLIT> := split <E> as <VAR>, <VAR> { <E> }
<FREE>  := free <E>; <E>
<APP>   := ( <E> <E> )
<DEF>   := def <VAR> : <T> = <E>; (for REPL use only)
<ENV>   := env; <E> (for output the environment)

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
AST:
...
...

[Type]
lin bool
[Evaluation]
false
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
lin bool
[Evaluation]
true
>> (lin fn x : lin bool {
    if x {
        un <un true, un false>
    } else {
        un <un false, un true>
    }
} x)
[Type]
un (un bool * un bool)
[Evaluation]
(true , false)
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