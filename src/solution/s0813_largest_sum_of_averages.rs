/**
 * [0813] Largest Sum of Averages
 *
 * You are given an integer array nums and an integer k. You can partition the array into at most k non-empty adjacent subarrays. The score of a partition is the sum of the averages of each subarray.
 * Note that the partition must use every integer in nums, and that the score is not necessarily an integer.
 * Return the maximum score you can achieve of all the possible partitions. Answers within 10^-6 of the actual answer will be accepted.
 *  
 * Example 1:
 *
 * Input: nums = [9,1,2,3,9], k = 3
 * Output: 20.00000
 * Explanation:
 * The best choice is to partition nums into [9], [1, 2, 3], [9]. The answer is 9 + (1 + 2 + 3) / 3 + 9 = 20.
 * We could have also partitioned nums into [9, 1], [2], [3, 9], for example.
 * That partition would lead to a score of 5 + 2 + 6 = 13, which is worse.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5,6,7], k = 4
 * Output: 20.50000
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i] <= 10^4
 * 	1 <= k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-sum-of-averages/
// discuss: https://leetcode.com/problems/largest-sum-of-averages/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut dp = {
            let mut sum = 0;
            nums.iter()
                .enumerate()
                .map(|(i, x)| {
                    sum += x;
                    sum as f64 / (i + 1) as f64
                })
                .collect::<Vec<_>>()
        };

        let k = k as usize;

        for i in 1..k {
            for j in (i..nums.len()).rev() {
                let mut ttl = 0;
                for l in (i..=j).rev() {
                    ttl += nums[l];
                    let avg = ttl as f64 / (j - l + 1) as f64;
                    dp[j] = dp[j].max(avg + dp[l - 1]);
                }
            }
        }

        dp[nums.len() - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0813_example_1() {
        let nums = vec![9, 1, 2, 3, 9];
        let k = 3;
        let result = 20.00000;

        assert_f64_near!(Solution::largest_sum_of_averages(nums, k), result);
    }

    #[test]
    fn test_0813_example_2() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 4;
        let result = 20.50000;

        assert_f64_near!(Solution::largest_sum_of_averages(nums, k), result);
    }
}
