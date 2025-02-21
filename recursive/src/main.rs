mod factorial;
mod fibonacci;
mod polidrome;
mod towerofhanoi;

use factorial::*;

fn main() {
    println!("Hello, Recursive!");
    let n = 5;
    println!("Head Recursion Factorial of {} is {}", n, fact_head(n));
    println!("Tail Recursion Factorial of {} is {}", n, fact_tail(n));

    let n = 10;
    println!("Fibonacci for {} is {}", n, fibonacci::fbc_compute(n));


    let not_polidrome = "This is polidrome?";

    println!("Is '{}' Is polidrome ? {}", not_polidrome, polidrome::is_polidrome(not_polidrome));

    let polidrome = "A man, a plan, a canal, Panama!";
    println!("Is '{}' Is polidrome ? {}", polidrome, polidrome::is_polidrome(polidrome));
}


