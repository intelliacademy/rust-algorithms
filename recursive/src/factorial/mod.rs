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
    if n == 0 { 1 } else { n * fact_head(n - 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_tail() {
        assert_eq!(fact_tail(0), 1);
        assert_eq!(fact_tail(1), 1);
        assert_eq!(fact_tail(2), 2);
        assert_eq!(fact_tail(3), 6);
        assert_eq!(fact_tail(4), 24);
        assert_eq!(fact_tail(5), 120);
        assert_eq!(fact_tail(6), 720);
        assert_eq!(fact_tail(7), 5040);
        assert_eq!(fact_tail(8), 40320);
        assert_eq!(fact_tail(9), 362880);
        assert_eq!(fact_tail(10), 3628800);
    }

    #[test]
    fn test_fact_head() {
        assert_eq!(fact_head(0), 1);
        assert_eq!(fact_head(1), 1);
        assert_eq!(fact_head(2), 2);
        assert_eq!(fact_head(3), 6);
        assert_eq!(fact_head(4), 24);
        assert_eq!(fact_head(5), 120);
        assert_eq!(fact_head(6), 720);
        assert_eq!(fact_head(7), 5040);
        assert_eq!(fact_head(8), 40320);
        assert_eq!(fact_head(9), 362880);
        assert_eq!(fact_head(10), 3628800);
    }
}
