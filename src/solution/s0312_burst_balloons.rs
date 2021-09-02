/**
 * [312] Burst Balloons
 *
 * You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.
 * If you burst the i^th balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.
 * Return the maximum coins you can collect by bursting the balloons wisely.
 *  
 * Example 1:
 *
 * Input: nums = [3,1,5,8]
 * Output: 167
 * Explanation:
 * nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
 * coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
 * Example 2:
 *
 * Input: nums = [1,5]
 * Output: 10
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 500
 * 	0 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/burst-balloons/
// discuss: https://leetcode.com/problems/burst-balloons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut two_sides_1_nums = vec![1; nums.len() + 2];

        for i in 0..nums.len() {
            two_sides_1_nums[i + 1] = nums[i];
        }
        let n = two_sides_1_nums.len();

        // dp[i][j] presents that the max coins of two_sides_1_nums[i..j], so dp[1][N] is the answer
        let mut dp = vec![vec![0; n]; n];

        for k in 2..n {
            for left in 0..n - k {
                let right = left + k;
                for i in (left + 1)..right {
                    dp[left][right] = std::cmp::max(
                        dp[left][right],
                        two_sides_1_nums[left] * two_sides_1_nums[i] * two_sides_1_nums[right]
                            + dp[left][i]
                            + dp[i][right],
                    );
                }
            }
        }

        dp[0][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0312_example_1() {
        let nums = vec![3, 1, 5, 8];
        let result = 167;

        assert_eq!(Solution::max_coins(nums), result);
    }

    #[test]
    fn test_0312_example_2() {
        let nums = vec![1, 5];
        let result = 10;

        assert_eq!(Solution::max_coins(nums), result);
    }
}
