/**
 * [0969] Pancake Sorting
 *
 * Given an array of integers arr, sort the array by performing a series of pancake flips.
 * In one pancake flip we do the following steps:
 *
 * 	Choose an integer k where 1 <= k <= arr.length.
 * 	Reverse the sub-array arr[0...k-1] (0-indexed).
 *
 * For example, if arr = [3,2,1,4] and we performed a pancake flip choosing k = 3, we reverse the sub-array [3,2,1], so arr = [<u>1</u>,<u>2</u>,<u>3</u>,4] after the pancake flip at k = 3.
 * Return an array of the k-values corresponding to a sequence of pancake flips that sort arr. Any valid answer that sorts the array within 10 * arr.length flips will be judged as correct.
 *  
 * Example 1:
 *
 * Input: arr = [3,2,4,1]
 * Output: [4,2,4,3]
 * Explanation:
 * We perform 4 pancake flips, with k values 4, 2, 4, and 3.
 * Starting state: arr = [3, 2, 4, 1]
 * After 1st flip (k = 4): arr = [<u>1</u>, <u>4</u>, <u>2</u>, <u>3</u>]
 * After 2nd flip (k = 2): arr = [<u>4</u>, <u>1</u>, 2, 3]
 * After 3rd flip (k = 4): arr = [<u>3</u>, <u>2</u>, <u>1</u>, <u>4</u>]
 * After 4th flip (k = 3): arr = [<u>1</u>, <u>2</u>, <u>3</u>, 4], which is sorted.
 *
 * Example 2:
 *
 * Input: arr = [1,2,3]
 * Output: []
 * Explanation: The input is already sorted, so there is no need to flip anything.
 * Note that other answers, such as [3, 3], would also be accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 100
 * 	1 <= arr[i] <= arr.length
 * 	All integers in arr are unique (i.e. arr is a permutation of the integers from 1 to arr.length).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pancake-sorting/
// discuss: https://leetcode.com/problems/pancake-sorting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut result = vec![];
        let n = arr.len();
        for i in 0..n {
            let (max_position, _) = arr[0..(n - i)]
                .iter()
                .enumerate()
                .max_by(|&(_, x), &(_, y)| x.cmp(y))
                .unwrap();
            result.push(max_position as i32 + 1);
            result.push((n - i) as i32);
            arr[0..=max_position].reverse();
            arr[0..(n - i)].reverse();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0969_example_1() {
        let arr = vec![3, 2, 4, 1];
        let result = vec![4, 2, 4, 3];

        assert_eq!(Solution::pancake_sort(arr), result);
    }

    #[test]
    #[ignore]
    fn test_0969_example_2() {
        let arr = vec![1, 2, 3];
        let result = vec![];

        assert_eq!(Solution::pancake_sort(arr), result);
    }
}
