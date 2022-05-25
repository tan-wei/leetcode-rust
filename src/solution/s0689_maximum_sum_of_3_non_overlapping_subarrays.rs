/**
 * [0689] Maximum Sum of 3 Non-Overlapping Subarrays
 *
 * Given an integer array nums and an integer k, find three non-overlapping subarrays of length k with maximum sum and return them.
 * Return the result as a list of indices representing the starting position of each interval (0-indexed). If there are multiple answers, return the lexicographically smallest one.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,2,6,7,5,1], k = 2
 * Output: [0,3,5]
 * Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
 * We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.
 *
 * Example 2:
 *
 * Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
 * Output: [0,2,4]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	1 <= nums[i] < 2^16
 * 	1 <= k <= floor(nums.length / 3)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
// discuss: https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const SUBARRAYS_COUNT: usize = 3;

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/discuss/647362/Rust-Clean-O(n)-solution.-Runs-in-4ms-and-with-3.3mb-usage
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let prefix_sum = Self::build_prefix_sum(&nums);
        let dp = Self::build_dp(&prefix_sum, k as usize, SUBARRAYS_COUNT);
        Self::build_result(&dp, k as usize)
    }

    fn build_prefix_sum(nums: &[i32]) -> Vec<i32> {
        std::iter::repeat(&0)
            .take(1)
            .chain(nums.iter())
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .collect()
    }

    fn build_dp(prefix_sums: &[i32], k: usize, levels: usize) -> Vec<Vec<i32>> {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; levels + 1]; prefix_sums.len()];

        for idx in k..dp.len() {
            for j in 1..=levels {
                let cur = dp[idx - k][j - 1] + prefix_sums[idx] - prefix_sums[idx - k];
                let prev = dp[idx - 1][j];
                dp[idx][j] = cur.max(prev);
            }
        }

        dp
    }

    fn build_result(dp: &Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0, 0, 0];
        let mut idx = dp.len() - 1;

        for j in (1..4).rev() {
            while dp[idx][j] == dp[idx - 1][j] {
                idx -= 1;
            }

            idx -= k;
            result[j - 1] = idx as i32;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0689_example_1() {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let k = 2;
        let result = vec![0, 3, 5];

        assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), result);
    }

    #[test]
    fn test_0689_example_2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        let k = 2;
        let result = vec![0, 2, 4];

        assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), result);
    }
}
