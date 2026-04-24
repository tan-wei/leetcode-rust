/**
 * [2441] Largest Positive Integer That Exists With Its Negative
 *
 * Given an integer array nums that does not contain any zeros, find the largest positive integer k such that -k also exists in the array.
 * Return the positive integer k. If there is no such integer, return -1.
 *  
 * Example 1:
 *
 * Input: nums = [-1,2,-3,3]
 * Output: 3
 * Explanation: 3 is the only valid k we can find in the array.
 *
 * Example 2:
 *
 * Input: nums = [-1,10,6,7,-7,1]
 * Output: 7
 * Explanation: Both 1 and 7 have their corresponding negative values in the array. 7 has a larger value.
 *
 * Example 3:
 *
 * Input: nums = [-10,8,6,7,-2,-3]
 * Output: -1
 * Explanation: There is no a single valid k, we return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	-1000 <= nums[i] <= 1000
 * 	nums[i] != 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
// discuss: https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .into_iter()
            .fold(vec![0i8; 2001], |mut v, x| {
                v[x.abs() as usize] += 1;
                v
            })
            .into_iter()
            .enumerate()
            .filter(|(_, x)| *x == 2)
            .map(|(i, _)| i as i32)
            .max()
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2441_example_1() {
        let nums = vec![-1, 2, -3, 3];

        let result = 3;

        assert_eq!(Solution::find_max_k(nums), result);
    }

    #[test]
    fn test_2441_example_2() {
        let nums = vec![-1, 10, 6, 7, -7, 1];

        let result = 7;

        assert_eq!(Solution::find_max_k(nums), result);
    }

    #[test]
    fn test_2441_example_3() {
        let nums = vec![-10, 8, 6, 7, -2, -3];

        let result = -1;

        assert_eq!(Solution::find_max_k(nums), result);
    }
}
