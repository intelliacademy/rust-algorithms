
pub fn fib(value: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c:i32;

    for _ in 2..=value {
        c = a + b;
        a = b;
        b = c;
    }
    b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib(){

    }
}