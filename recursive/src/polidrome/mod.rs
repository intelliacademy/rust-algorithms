pub fn is_polidrome(s: &str) -> bool {
    let s = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

pub fn is_polidrome_recursive(s: &str) -> bool {
    let s = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();

    fn is_polidrome_recursive_inner(s: &Vec<char>, i: usize, j: usize) -> bool {
        if i >= j {
            return true;
        }
        if s[i] != s[j] {
            return false;
        }
        is_polidrome_recursive_inner(s, i + 1, j - 1)
    }

    is_polidrome_recursive_inner(&s, 0, s.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_polidrome() {
        let not_polidrome = "This is polidrome?";
        assert_eq!(is_polidrome(not_polidrome), false);
    }

    #[test]
    fn test_is_polidrome_recursive() {
        let polidrome = "A man, a plan, a canal, Panama!";
        assert_eq!(is_polidrome_recursive(polidrome), true);
    }
}
