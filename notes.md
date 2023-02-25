# Chapter 1: Getting started

* fn to declare a function
* ! denotes a macro
* rustc for compiling
* Cargo.toml
  * edition = rust [edition](https://doc.rust-lang.org/book/appendix-05-editions.html) to use
* cargo build
  * cargo build --release, enables optimizations but longer compile time
* cargo run
* cargo check

# Chapter 2: Programming a Guessing Game

* `use std::io` - Bring io library into scope.
* `let x = 5;` - Declare a variable.
* `let mut x = 5;` - Declare a mutable variable.
* `// comment`
* `String::new()` - Associated function, `new`, implemented on type `String`
  * `new()` is often used as a quasi-constructor like in Go
* `&guess` denotes a reference
  * `&mut guess` is a mutable reference
* [Result](https://doc.rust-lang.org/std/result/enum.Result.html) is an enum, typically `Ok` or `Err`
  * `Result.expect()` used to pluck out return value and panic if error occurs.
* Placeholders: `println!("Test {variable} here")`
* Adding a dependency
  * Add to `dependencies` section in Cargo.toml
  * Run `cargo build`
* Update dependencies in `Cargo.toml` with `cargo update`
* `cargo doc --open` builds docs for dependencies and opens them in browser
* [match](https://doc.rust-lang.org/book/ch06-02-match.html) expressions
  * match expression is made up of "arms".
* You can shadow a variable of the same name but different types.
* `loop` keyword creates an infinite loop.
* `_` param value is a catchall.