/**
 * [2407] Longest Increasing Subsequence II
 *
 * You are given an integer array nums and an integer k.
 * Find the longest subsequence of nums that meets the following requirements:
 *
 * 	The subsequence is strictly increasing and
 * 	The difference between adjacent elements in the subsequence is at most k.
 *
 * Return the length of the longest subsequence that meets the requirements.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: nums = [4,2,1,4,3,4,5,8,15], k = 3
 * Output: 5
 * Explanation:
 * The longest subsequence that meets the requirements is [1,3,4,5,8].
 * The subsequence has a length of 5, so we return 5.
 * Note that the subsequence [1,3,4,5,8,15] does not meet the requirements because 15 - 8 = 7 is larger than 3.
 *
 * Example 2:
 *
 * Input: nums = [7,4,5,1,8,12,4,7], k = 5
 * Output: 4
 * Explanation:
 * The longest subsequence that meets the requirements is [4,5,8,12].
 * The subsequence has a length of 4, so we return 4.
 *
 * Example 3:
 *
 * Input: nums = [1,5], k = 1
 * Output: 1
 * Explanation:
 * The longest subsequence that meets the requirements is [1].
 * The subsequence has a length of 1, so we return 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i], k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-increasing-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2407_example_1() {
        let nums = vec![4, 2, 1, 4, 3, 4, 5, 8, 15];
        let k = 3;

        let result = 5;

        assert_eq!(Solution::length_of_lis(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2407_example_2() {
        let nums = vec![7, 4, 5, 1, 8, 12, 4, 7];
        let k = 5;

        let result = 4;

        assert_eq!(Solution::length_of_lis(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2407_example_3() {
        let nums = vec![1, 5];
        let k = 1;

        let result = 1;

        assert_eq!(Solution::length_of_lis(nums, k), result);
    }
}
