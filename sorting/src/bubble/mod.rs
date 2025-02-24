pub fn sort_by_bubble<T>(arr: &mut Vec<T>) -> &Vec<T>
where
    T: Ord,
{
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_bubble() {
        let mut arr = vec![4, 3, 2, 1];
        let sorted_arr = sort_by_bubble(&mut arr);
        assert_eq!(sorted_arr.to_vec(), vec![1, 2, 3, 4].to_vec());
    }
}
