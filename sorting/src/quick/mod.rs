

pub fn sort_by_quick<T>(arr: &mut Vec<T>) -> &Vec<T>
where T: Ord + Clone
{
    fn quick_sort<T: Ord + std::clone::Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = partition(&mut arr.to_vec());
        quick_sort(&mut arr[0..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
    arr
}

fn partition<T>(arr: &mut Vec<T>) -> usize
where T: Ord
{
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..arr.len() {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
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