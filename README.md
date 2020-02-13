# Rust implementation of the famous Monkey programming language.


## Monkey; The programming language that lives in books
<img src="img/logo.png" width="200px" height="200px"/>

This is a Rust implementation of the original book [Writing An Interpreter In Go](https://interpreterbook.com/),
which (as you've might have guessed) is written in Go. At the moment this is a Work In Progress. The current
implementation has:

- [x] Integers, Booleans, Strings, Arrays, HashMaps
- [x] A REPL
- [x] Arithmetic expressions
- [x] Let statements
- [x] First-class and higher-order functions
- [x] A few Built-in functions
- [x] Recursion
- [x] Closures

### Starting the REPL:

`$ cargo run`

### Excerpt of the Monkey Language
```
let fibonacci = fn(x) {
  if (x == 0) {
    0
  } else {
    if (x == 1) {
      return 1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};
```