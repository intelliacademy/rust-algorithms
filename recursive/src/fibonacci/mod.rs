pub fn fbc_compute(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fbc_compute(n - 1) + fbc_compute(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fbc_compute() {
        assert_eq!(fbc_compute(0), 0);
        assert_eq!(fbc_compute(1), 1);
        assert_eq!(fbc_compute(2), 1);
        assert_eq!(fbc_compute(3), 2);
        assert_eq!(fbc_compute(4), 3);
        assert_eq!(fbc_compute(5), 5);
        assert_eq!(fbc_compute(6), 8);
        assert_eq!(fbc_compute(7), 13);
        assert_eq!(fbc_compute(8), 21);
        assert_eq!(fbc_compute(9), 34);
        assert_eq!(fbc_compute(10), 55);
    }
}
