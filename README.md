RustCalc
========

RustCalc is a polish notation calculator (see http://en.wikipedia.org/wiki/Polish_notation). This means that you must write the operation first and the arguments later, like in `(+ 2 3)` or `(- 10 (* 2 6) 5)`. The reason of this is that it is easier to parse.

## Compiling

To compile this you must use the latest nightly build of Rust. Just use `rustc main.rs` and it should work.

## Goals

After seeing https://github.com/libfud/rcalc, I thought that it would be a good idea to write a little calculator in Rust. However, my focus is mainly placed on making a parser and generating an AST (Abstract Syntax Tree), rather than having a good calculator, so don't expect features like arbitrary precision and such things. I have written it from scratch, trying to do it in an idiomatic way, so I hope it can also be useful for the ones wanting to learn Rust.

## Future

Since this is mostly an experiment, the future is uncertain. At this moment I am planning to add the following features:

* Support for predefined functions (like `sqrt`, `pow`, etc) and constants (like `pi` and `e`).
* Unit tests