# Notes on Rust Programming Language

### Variables

- Rust variables are immutable by default, meaning once we assign a value to the variable, the value won't change
- You can make a variable mutable by using `let mut ex_var = 5`
- Rust allows _shadowing_ to reuse a variable name but assign it a different type (which is what it's mainly used for)
- Shadowing is also good for changing a variable's value but _not_ changing the mutability (keep it mutable by using `let` keyword)
- You can use `_` to visually separate variables in rust (like a 32bit hex number)

### Error Handling
- Overflows: Rust can handle overflowing vars w/ "wrapping", using a few different methods (see the book)
- Rust can handle invalid indexing as a runtime error (unlike C, for example)

### Function vs Macro calling

Using a '!' in a function call, calls the _macro_ instead of a function (e.g. println!())

### Cargo

- Cargo is Rust's build system and package manager

- Using 'cargo check' is faster than 'cargo build' because it doesn't produce and exec. image, so you can constantly check your code to make sure you're writing good code 
- Documentation for any crates (dependencies) listed in your Cargo.toml file can be generated with `cargo doc --open`
- create a new project with cargo by using `cargo new <name> --vcs none`

### Crates

- A crate is a collection of source code files, for example, a _library crate_ like `rand` is a crate that contains code intended to be used within a program, but not on its own (like a library in a different language)
- Crates.io is where people in the Rust ecosystem store their open-source projects for others to use
