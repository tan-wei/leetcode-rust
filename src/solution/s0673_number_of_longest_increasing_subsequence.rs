/**
 * [0673] Number of Longest Increasing Subsequence
 *
 * Given an integer array nums, return the number of longest increasing subsequences.
 * Notice that the sequence has to be strictly increasing.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,5,4,7]
 * Output: 2
 * Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
 *
 * Example 2:
 *
 * Input: nums = [2,2,2,2,2]
 * Output: 5
 * Explanation: The length of longest continuous increasing subsequence is 1, and there are 5 subsequences' length is 1, so output 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2000
 * 	-10^6 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-longest-increasing-subsequence/
// discuss: https://leetcode.com/problems/number-of-longest-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_len = 0;

        // dp[i]: (length, number of LIS which ends with nums[i])
        let mut dp = vec![(1, 1); nums.len()];

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[i].0 == dp[j].0 + 1 {
                        dp[i].1 += dp[j].1;
                    }

                    if dp[i].0 < dp[j].0 + 1 {
                        dp[i] = (dp[j].0 + 1, dp[j].1);
                    }
                }
            }

            if max_len == dp[i].0 {
                result += dp[i].1;
            }
            if max_len < dp[i].0 {
                max_len = dp[i].0;
                result = dp[i].1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0673_example_1() {
        let nums = vec![1, 3, 5, 4, 7];
        let result = 2;

        assert_eq!(Solution::find_number_of_lis(nums), result);
    }

    #[test]
    fn test_0673_example_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let result = 5;

        assert_eq!(Solution::find_number_of_lis(nums), result);
    }
}
