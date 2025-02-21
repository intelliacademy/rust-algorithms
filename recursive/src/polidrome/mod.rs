

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
            return false
        }
        i += 1;
        j -= 1;
    }
    true
}