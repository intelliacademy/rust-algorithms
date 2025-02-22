//Sum Triangle of Array solve with recursive function
/**
    * Given a triangle of numbers, find the maximum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.
    * For example, given the following triangle
    * [
    * [1],
    * [2, 3],
    * [4, 5, 6],
    * [7, 8, 9, 10]
    * ]
    * The maximum path sum from top to bottom is 20 (1->3->6->10)

*/


pub fn sum_triangle(arr: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i == arr.len() - 1 {
        return arr[i][j];
    }
    let left = sum_triangle(arr, i + 1, j);
    let right = sum_triangle(arr, i + 1, j + 1);
    arr[i][j] + std::cmp::max(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_triangle() {
        let arr = vec![
            vec![1],
            vec![2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9, 10],
        ];
        assert_eq!(sum_triangle(&arr, 0, 0), 20);
    }
}