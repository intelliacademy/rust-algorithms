
pub fn fbc_compute(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fbc_compute(n - 1) + fbc_compute(n - 2)
    }
}