use compiler::compiler::compiler::Compiler;
use compiler::utils::compile;
use compiler::vm::vm::{run_vm, VM};
use std::time::{Duration, Instant};

fn main() {
    let input = "let fibonacci = fn(x) {
  if (x == 0) {
    0
  } else {
    if (x == 1) {
      return 1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
}; fibonacci(35)";
    let com = compile(&input).unwrap();
    let bytecode = com.bytecode();

    let now = Instant::now();
    run_vm(&bytecode);
    println!("{}", now.elapsed().as_millis());
}
