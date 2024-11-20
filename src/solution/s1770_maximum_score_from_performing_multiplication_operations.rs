/**
 * [1770] Maximum Score from Performing Multiplication Operations
 *
 * You are given two 0-indexed integer arrays nums and multipliers of size n and m respectively, where n >= m.
 * You begin with a score of 0. You want to perform exactly m operations. On the i^th operation (0-indexed) you will:
 *
 *     Choose one integer x from either the start or the end of the array nums.
 *     Add multipliers[i] * x to your score.
 *     
 *         Note that multipliers[0] corresponds to the first operation, multipliers[1] to the second operation, and so on.
 *     
 *     
 *     Remove x from nums.
 *
 * Return the maximum score after performing m operations.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3], multipliers = [3,2,1]
 * Output: 14
 * Explanation: An optimal solution is as follows:
 * - Choose from the end, [1,2,<u>3</u>], adding 3 * 3 = 9 to the score.
 * - Choose from the end, [1,<u>2</u>], adding 2 * 2 = 4 to the score.
 * - Choose from the end, [<u>1</u>], adding 1 * 1 = 1 to the score.
 * The total score is 9 + 4 + 1 = 14.
 * Example 2:
 *
 * Input: nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
 * Output: 102
 * Explanation: An optimal solution is as follows:
 * - Choose from the start, [<u>-5</u>,-3,-3,-2,7,1], adding -5 * -10 = 50 to the score.
 * - Choose from the start, [<u>-3</u>,-3,-2,7,1], adding -3 * -5 = 15 to the score.
 * - Choose from the start, [<u>-3</u>,-2,7,1], adding -3 * 3 = -9 to the score.
 * - Choose from the end, [-2,7,<u>1</u>], adding 1 * 4 = 4 to the score.
 * - Choose from the end, [-2,<u>7</u>], adding 7 * 6 = 42 to the score.
 * The total score is 50 + 15 - 9 + 4 + 42 = 102.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	m == multipliers.length
 * 	1 <= m <= 300
 * 	m <= n <= 10^5
 * 	-1000 <= nums[i], multipliers[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/
// discuss: https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/solutions/2455163/rust-bottom-up-and-top-down-dp-with-comments/
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![0; m + 1]; m + 1];

        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                let multiplier = multipliers[left + right];
                dp[left][right] = (dp[left + 1][right] + multiplier * nums[left])
                    .max(dp[left][right + 1] + multiplier * nums[n - right - 1]);
            }
        }

        dp[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1770_example_1() {
        let nums = vec![1, 2, 3];
        let multipliers = vec![3, 2, 1];

        let result = 14;

        assert_eq!(Solution::maximum_score(nums, multipliers), result);
    }

    #[test]
    fn test_1770_example_2() {
        let nums = vec![-5, -3, -3, -2, 7, 1];
        let multipliers = vec![-10, -5, 3, 4, 6];

        let result = 102;

        assert_eq!(Solution::maximum_score(nums, multipliers), result);
    }
}
