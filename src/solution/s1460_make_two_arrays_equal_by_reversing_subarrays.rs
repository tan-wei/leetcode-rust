/**
 * [1460] Make Two Arrays Equal by Reversing Subarrays
 *
 * You are given two integer arrays of equal length target and arr. In one step, you can select any non-empty subarray of arr and reverse it. You are allowed to make any number of steps.
 * Return true if you can make arr equal to target or false otherwise.
 *  
 * Example 1:
 *
 * Input: target = [1,2,3,4], arr = [2,4,1,3]
 * Output: true
 * Explanation: You can follow the next steps to convert arr to target:
 * 1- Reverse subarray [2,4,1], arr becomes [1,4,2,3]
 * 2- Reverse subarray [4,2], arr becomes [1,2,4,3]
 * 3- Reverse subarray [4,3], arr becomes [1,2,3,4]
 * There are multiple ways to convert arr to target, this is not the only way to do so.
 *
 * Example 2:
 *
 * Input: target = [7], arr = [7]
 * Output: true
 * Explanation: arr is equal to target without any reverses.
 *
 * Example 3:
 *
 * Input: target = [3,7,9], arr = [3,7,11]
 * Output: false
 * Explanation: arr does not have value 9 and it can never be converted to target.
 *
 *  
 * Constraints:
 *
 * 	target.length == arr.length
 * 	1 <= target.length <= 1000
 * 	1 <= target[i] <= 1000
 * 	1 <= arr[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/
// discuss: https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target = target;
        let mut arr = arr;

        target.sort_unstable();
        arr.sort_unstable();

        arr == target
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1460_example_1() {
        let target = vec![1, 2, 3, 4];
        let arr = vec![2, 4, 1, 3];

        let result = true;

        assert_eq!(Solution::can_be_equal(target, arr), result);
    }

    #[test]
    fn test_1460_example_2() {
        let target = vec![7];
        let arr = vec![7];

        let result = true;

        assert_eq!(Solution::can_be_equal(target, arr), result);
    }

    #[test]
    fn test_1460_example_3() {
        let target = vec![3, 7, 9];
        let arr = vec![3, 7, 11];

        let result = false;

        assert_eq!(Solution::can_be_equal(target, arr), result);
    }
}
