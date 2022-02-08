/**
 * [0518] Coin Change 2
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
 * Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
 * You may assume that you have an infinite number of each kind of coin.
 * The answer is guaranteed to fit into a signed 32-bit integer.
 *  
 * Example 1:
 *
 * Input: amount = 5, coins = [1,2,5]
 * Output: 4
 * Explanation: there are four ways to make up the amount:
 * 5=5
 * 5=2+2+1
 * 5=2+1+1+1
 * 5=1+1+1+1+1
 *
 * Example 2:
 *
 * Input: amount = 3, coins = [2]
 * Output: 0
 * Explanation: the amount of 3 cannot be made up just with coins of 2.
 *
 * Example 3:
 *
 * Input: amount = 10, coins = [10]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= coins.length <= 300
 * 	1 <= coins[i] <= 5000
 * 	All the values of coins are unique.
 * 	0 <= amount <= 5000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coin-change-2/
// discuss: https://leetcode.com/problems/coin-change-2/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];

        dp[0] = 1;

        for i in 0..coins.len() {
            for j in 1..=amount {
                if j < coins[i] as usize {
                    continue;
                }

                dp[j] += dp[j - coins[i] as usize];
            }
        }

        dp[amount]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0518_example_1() {
        let amount = 5;
        let coins = vec![1, 2, 5];
        let result = 4;

        assert_eq!(Solution::change(amount, coins), result);
    }

    #[test]
    fn test_0518_example_2() {
        let amount = 3;
        let coins = vec![2];
        let result = 0;

        assert_eq!(Solution::change(amount, coins), result);
    }

    #[test]
    fn test_0518_example_3() {
        let amount = 10;
        let coins = vec![10];
        let result = 1;

        assert_eq!(Solution::change(amount, coins), result);
    }
}
