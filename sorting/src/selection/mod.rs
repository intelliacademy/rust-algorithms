/**
    [Selection Sort](https://en.wikipedia.org/wiki/Selection_sort)
    is an in-place comparison sorting algorithm that divides the input list into two parts:
    the sublist of items already sorted, which is built up from left to right at the front (left) of the list,
    Example usage:
    ```rust
    use sorting::selection::selection_sort;
    let mut arr = [64, 25, 12, 22, 11];
    selection_sort(&mut arr);
    assert_eq!(arr, [11, 12, 22, 25, 64]);
    ```
*/

pub fn sort_by_selection<T>(arr: &mut Vec<T>) -> &Vec<T>
where
    T: Ord,
{
    for i in 0..arr.len() - 1 {
        let mut smallest = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        arr.swap(i, smallest);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![64, 25, 12, 22, 11];
        let sorted_arr = sort_by_selection(&mut arr);
        assert_eq!(sorted_arr.to_vec(), [11, 12, 22, 25, 64]);
    }
}
