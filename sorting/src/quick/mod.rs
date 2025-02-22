

pub fn sort_by_quick<T>(arr: &mut Vec<T>) -> &Vec<T> {
    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![64, 25, 12, 22, 11];
        let sorted_arr = sort_by_quick(&mut arr);
        assert_eq!(sorted_arr.to_vec(), [11, 12, 22, 25, 64]);
    }
}