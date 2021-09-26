/**
 * [343] Integer Break
 *
 * Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
 * Return the maximum product you can get.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 1
 * Explanation: 2 = 1 + 1, 1 &times; 1 = 1.
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 36
 * Explanation: 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 58
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-break/
// discuss: https://leetcode.com/problems/integer-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 1;

        for i in 3..n + 1 {
            for j in 1..i {
                dp[i] = dp[i].max(dp[j].max(j) * (i - j));
            }
        }
        dp[n] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0343_example_1() {
        let n = 2;
        let result = 1;

        assert_eq!(Solution::integer_break(n), result);
    }

    #[test]
    fn test_0343_example_2() {
        let n = 10;
        let result = 36;

        assert_eq!(Solution::integer_break(n), result);
    }
}
