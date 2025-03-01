//Longest Common Subsequence

pub fn lcs_resolve(a: &str, b: &str) -> Option<String> {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let (len_a, len_b) = (a.len(), b.len());

    let mut solution = vec![vec![0, len_b + 1]; len_a + 1];

    for (i, mi) in a.iter().enumerate() {
        for (j, mj) in b.iter().enumerate() {
            solution[i + 1][j + 1] = if mi == mj {
                solution[i][j] + 1
            } else {
                solution[i][j + 1].max(solution[i + 1][j])
            }
        }
    }

    let mut result: Vec<char> = Vec::new();

    let (mut i, mut j) = (len_a, len_b);
    while i > 0 && j > 0 {}
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {}
}
