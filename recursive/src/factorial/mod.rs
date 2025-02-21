pub fn fact_tail(n: u64) -> u64 {
    fn fact_iter(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            fact_iter(n - 1, n * acc)
        }
    }
    fact_iter(n, 1)
}

//head recursive
pub fn fact_head(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * fact_head(n - 1)
    }
}