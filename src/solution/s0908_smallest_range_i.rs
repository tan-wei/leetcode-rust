/**
 * [0908] Smallest Range I
 *
 * You are given an integer array nums and an integer k.
 * In one operation, you can choose any index i where 0 <= i < nums.length and change nums[i] to nums[i] + x where x is an integer from the range [-k, k]. You can apply this operation at most once for each index i.
 * The score of nums is the difference between the maximum and minimum elements in nums.
 * Return the minimum score of nums after applying the mentioned operation at most once for each index in it.
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
 * Output: 0
 * Explanation: Change nums to be [4, 4, 4]. The score is max(nums) - min(nums) = 4 - 4 = 0.
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

// problem: https://leetcode.com/problems/smallest-range-i/
// discuss: https://leetcode.com/problems/smallest-range-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        std::cmp::max(
            nums.iter().max().unwrap() - nums.iter().min().unwrap() - 2 * k,
            0,
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0908_example_1() {
        let nums = vec![1];
        let k = 0;
        let result = 0;

        assert_eq!(Solution::smallest_range_i(nums, k), result);
    }

    #[test]
    fn test_0908_example_2() {
        let nums = vec![0, 10];
        let k = 2;
        let result = 6;

        assert_eq!(Solution::smallest_range_i(nums, k), result);
    }

    #[test]
    fn test_0908_example_3() {
        let nums = vec![1, 3, 6];
        let k = 3;
        let result = 0;

        assert_eq!(Solution::smallest_range_i(nums, k), result);
    }
}
