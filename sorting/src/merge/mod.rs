
pub fn sort_by_merge<T>(arr: &mut Vec<T>) -> Vec<T>
    where T: Ord + Clone
{
    if arr.len() <= 1 {
        return arr.to_vec();
    }else {
        let mid = arr.len() / 2;
        let mut left = arr[0..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        sort_by_merge(&mut left);
        sort_by_merge(&mut right);
        merge(arr, mid);
    }
    arr.to_vec()
}

fn merge<T>(arr: &mut [T], mid: usize)
where
    T: Clone + Ord,
{
    let left = arr[0..mid].to_vec();
    let right = arr[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] <= right[r]) {
            *val = left[l].clone();
            l += 1;
        } else {
            *val = right[r].clone();
            r += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![64, 25, 12, 22, 11];
        let sorted_arr = sort_by_merge(&mut arr);
        assert_eq!(sorted_arr.to_vec(), [11, 12, 22, 25, 64], "Failed to sort by merge");
    }

}