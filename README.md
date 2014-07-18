RustCalc
========

RustCalc is a polish notation calculator (see http://en.wikipedia.org/wiki/Polish_notation). This means that you must write the operation first and the arguments later, like in `(+ 2 3)` or `(- 10 (* 2 6) 5)`. The reason of this is that it is easier to parse.

## Compiling

To compile this you must use the latest nightly build of Rust. Just use `rustc main.rs` and it should work.

## Goals

After seeing https://github.com/libfud/rcalc, I thought that it would be a good idea to write a little calculator in Rust. However, my focus is mainly placed on making a parser and generating an AST (Abstract Syntax Tree), rather than having a good calculator, so don't expect features like arbitrary precision and such things. I have written it from scratch, trying to do it in an idiomatic way, so I hope it can also be useful for the ones wanting to learn Rust.

## Features

After finishing the parser I thought it would be interesting to enhance the calculator with some features. These are some of them:

* Predefined functions (`sqrt` and `pow`). Example: `(pow 2 8)`.
* Predefined constants (`pi` and `e`). Example: `(* pi 2)`.
* User-defined variables. Example: `(set myVar 42)`, `(+ myVar 3)`.
* `If` statements and comparison operators. Example: `(if (== 2 2) (+ 5 8) (- 5 8))`. Actually, `if` is a function that takes the first argument as the condition and returns the second if the condition is true. Otherwise it returns the third parameter.