/**
 * [1425] Constrained Subsequence Sum
 *
 * Given an integer array nums and an integer k, return the maximum sum of a non-empty subsequence of that array such that for every two consecutive integers in the subsequence, nums[i] and nums[j], where i < j, the condition j - i <= k is satisfied.
 * A subsequence of an array is obtained by deleting some number of elements (can be zero) from the array, leaving the remaining elements in their original order.
 *  
 * Example 1:
 *
 * Input: nums = [10,2,-10,5,20], k = 2
 * Output: 37
 * Explanation: The subsequence is [10, 2, 5, 20].
 *
 * Example 2:
 *
 * Input: nums = [-1,-2,-3], k = 1
 * Output: -1
 * Explanation: The subsequence must be non-empty, so we choose the largest number.
 *
 * Example 3:
 *
 * Input: nums = [10,-2,-10,-5,20], k = 2
 * Output: 23
 * Explanation: The subsequence is [10, -2, -5, 20].
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/constrained-subsequence-sum/
// discuss: https://leetcode.com/problems/constrained-subsequence-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/constrained-subsequence-sum/solutions/3111911/just-a-runnable-solution/
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut result = nums[0];
        let mut q = std::collections::VecDeque::new();

        for i in 0..nums.len() {
            dp[i] = nums[i];
            if let Some(&j) = q.front() {
                dp[i] = std::cmp::max(dp[i], dp[j] + nums[i]);
            }
            result = std::cmp::max(result, dp[i]);
            while let Some(&j) = q.back() {
                if dp[j] < dp[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(i);
            if let Some(&j) = q.front() {
                if i as i32 - j as i32 >= k {
                    q.pop_front();
                }
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
    fn test_1425_example_1() {
        let nums = vec![10, 2, -10, 5, 20];
        let k = 2;

        let result = 37;

        assert_eq!(Solution::constrained_subset_sum(nums, k), result);
    }

    #[test]
    fn test_1425_example_2() {
        let nums = vec![-1, -2, -3];
        let k = 1;

        let result = -1;

        assert_eq!(Solution::constrained_subset_sum(nums, k), result);
    }

    #[test]
    fn test_1425_example_3() {
        let nums = vec![10, -2, -10, -5, 20];
        let k = 2;

        let result = 23;

        assert_eq!(Solution::constrained_subset_sum(nums, k), result);
    }
}
