/**
 * [0413] Arithmetic Slices
 *
 * An integer array is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
 *
 * 	For example, [1,3,5,7,9], [7,7,7,7], and [3,-1,-5,-9] are arithmetic sequences.
 *
 * Given an integer array nums, return the number of arithmetic subarrays of nums.
 * A subarray is a contiguous subsequence of the array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: 3
 * Explanation: We have 3 arithmetic slices in nums: [1, 2, 3], [2, 3, 4] and [1,2,3,4] itself.
 *
 * Example 2:
 *
 * Input: nums = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	-1000 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arithmetic-slices/
// discuss: https://leetcode.com/problems/arithmetic-slices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut sum = 0;
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                dp[i] = 1 + dp[i - 1];
            }
            sum += dp[i]
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0413_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = 3;

        assert_eq!(Solution::number_of_arithmetic_slices(nums), result);
    }

    #[test]
    fn test_0413_example_2() {
        let nums = vec![1];
        let result = 0;

        assert_eq!(Solution::number_of_arithmetic_slices(nums), result);
    }
}
