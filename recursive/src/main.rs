mod factorial;
mod fibonacci;

use factorial::*;

fn main() {
    println!("Hello, Recursive!");
    let n = 5;
    println!("Head Recursion Factorial of {} is {}", n, fact_head(n));
    println!("Tail Recursion Factorial of {} is {}", n, fact_tail(n));

    let n = 10;
    println!("Fibonacci for {} is {}", n, fibonacci::fbc_compute(n));
}


