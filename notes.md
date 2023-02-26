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
* `let x = 5` - Declare a variable.
* `let mut x = 5` - Declare a mutable variable.
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

# Chapter 3: Common Programming Concepts

* `let mut x = 5` - Declare a mutable variable.
* `const SOME_VAR: u32 = 5`
* [Rust const reference](https://doc.rust-lang.org/reference/const_eval.html)
* Shadowing: `let x = 5; let x = x + 10;`
  * Can change the type of the variable since we're creating new.
* Inner scoping: curly braces within a function body.
* A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
* number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type.
* Number literals can also use _ as a visual separator to make the number easier to read, such as `1_000`.
* Literals:
  * Decimal `98_222`
  * Hex `0xff`
  * Octal `0o77`
  * Binary `0b1111_0000`
  * Byte (`u8` only) `b'A'`
* `isize` and `usize` types are machine-specific. Mainly used for sizing collections.
* Overflows
  * When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs
  * When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping
  * Use stdlib convenience functions: `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*`
* Floats
  * `f32` and `f64`
  * The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision
* Character type
  * We specify char literals with single quotes, as opposed to string literals, which use double quotes
  * Rust’s char type is four bytes in size and represents a Unicode Scalar Value
* Compound types
  * Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
  * Tuple
    * Fixed length
    * Members don't have to be of the same type
    * Destructure tuple: `let tup = (500, 6.4, 1); let (x, y, z) = tup;`
    * We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    * The tuple without any values has a special name, "unit". written `()`
  * Array
    * Every element of an array must have the same type
    * Fixed length
    * Arrays are useful when you want your data allocated on the stack rather than the heap
    * A *vector* is a similar collection type provided by the standard library that is allowed to grow or shrink in size
    * `let a: [i32; 5] = [1, 2, 3, 4, 5];` 5 is the size
    * `let a = [3; 5];` equivalent to `[3, 3, 3, 3, 3];`
    * Index errors lead to panics.
  * Functions
    * Use snake case
    * Statements are instructions that perform some action and do not return a value.
    * Expressions evaluate to a resultant value.
    * Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    * You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
  * Loops
    * `loop` loops infinitely
    * Break can return an arg `break 10`
    * You can label loops: `'counting_up: loop {`, `break 'counting_up;`