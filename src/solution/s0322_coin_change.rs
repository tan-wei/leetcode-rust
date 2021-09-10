/**
 * [322] Coin Change
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
 * Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
 * You may assume that you have an infinite number of each kind of coin.
 *  
 * Example 1:
 *
 * Input: coins = [1,2,5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 *
 * Example 2:
 *
 * Input: coins = [2], amount = 3
 * Output: -1
 *
 * Example 3:
 *
 * Input: coins = [1], amount = 0
 * Output: 0
 *
 * Example 4:
 *
 * Input: coins = [1], amount = 1
 * Output: 1
 *
 * Example 5:
 *
 * Input: coins = [1], amount = 2
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= coins.length <= 12
 * 	1 <= coins[i] <= 2^31 - 1
 * 	0 <= amount <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coin-change/
// discuss: https://leetcode.com/problems/coin-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/coin-change/discuss/1104496/Rust-DP-solution
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);
        for i in 1..=amount {
            dp[i] = coins
                .iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j <= i {
                        dp[i - j].map(|n| n + 1)
                    } else {
                        None
                    }
                })
                .min();
        }
        dp[amount].unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0322_example_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let result = 3;

        assert_eq!(Solution::coin_change(coins, amount), result)
    }

    #[test]
    fn test_0322_example_2() {
        let coins = vec![2];
        let amount = 3;
        let result = -1;

        assert_eq!(Solution::coin_change(coins, amount), result)
    }

    #[test]
    fn test_0322_example_3() {
        let coins = vec![1];
        let amount = 0;
        let result = 0;

        assert_eq!(Solution::coin_change(coins, amount), result)
    }

    #[test]
    fn test_0322_example_4() {
        let coins = vec![1];
        let amount = 1;
        let result = 1;

        assert_eq!(Solution::coin_change(coins, amount), result)
    }

    #[test]
    fn test_0322_example_5() {
        let coins = vec![1];
        let amount = 2;
        let result = 2;

        assert_eq!(Solution::coin_change(coins, amount), result)
    }
}
