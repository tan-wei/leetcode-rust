/**
 * [0910] Smallest Range II
 *
 * You are given an integer array nums and an integer k.
 * For each index i where 0 <= i < nums.length, change nums[i] to be either nums[i] + k or nums[i] - k.
 * The score of nums is the difference between the maximum and minimum elements in nums.
 * Return the minimum score of nums after changing the values at each index.
 *  
 * Example 1:
 *
 * Input: nums = [1], k = 0
 * Output: 0
 * Explanation: The score is max(nums) - min(nums) = 1 - 1 = 0.
 *
 * Example 2:
 *
 * Input: nums = [0,10], k = 2
 * Output: 6
 * Explanation: Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.
 *
 * Example 3:
 *
 * Input: nums = [1,3,6], k = 3
 * Output: 3
 * Explanation: Change nums to be [4, 6, 3]. The score is max(nums) - min(nums) = 6 - 3 = 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 10^4
 * 	0 <= k <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-range-ii/
// discuss: https://leetcode.com/problems/smallest-range-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut result = nums[n - 1] - nums[0];
        for i in 0..n - 1 {
            let (x, y) = (nums[i], nums[i + 1]);
            let high = std::cmp::max(nums[n - 1] - k, x + k);
            let low = std::cmp::min(nums[0] + k, y - k);
            result = std::cmp::min(result, high - low);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0910_example_1() {
        let nums = vec![1];
        let k = 0;
        let result = 0;

        assert_eq!(Solution::smallest_range_ii(nums, k), result);
    }

    #[test]
    fn test_0910_example_2() {
        let nums = vec![0, 10];
        let k = 2;
        let result = 6;

        assert_eq!(Solution::smallest_range_ii(nums, k), result);
    }

    #[test]
    fn test_0910_example_3() {
        let nums = vec![1, 3, 6];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::smallest_range_ii(nums, k), result);
    }
}
