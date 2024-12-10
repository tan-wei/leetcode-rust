/**
 * [1798] Maximum Number of Consecutive Values You Can Make
 *
 * You are given an integer array coins of length n which represents the n coins that you own. The value of the i^th coin is coins[i]. You can make some value x if you can choose some of your n coins such that their values sum up to x.
 * Return the maximum number of consecutive integer values that you can make with your coins starting from and including 0.
 * Note that you may have multiple coins of the same value.
 *  
 * Example 1:
 *
 * Input: coins = [1,3]
 * Output: 2
 * Explanation: You can make the following values:
 * - 0: take []
 * - 1: take [1]
 * You can make 2 consecutive integer values starting from 0.
 * Example 2:
 *
 * Input: coins = [1,1,1,4]
 * Output: 8
 * Explanation: You can make the following values:
 * - 0: take []
 * - 1: take [1]
 * - 2: take [1,1]
 * - 3: take [1,1,1]
 * - 4: take [4]
 * - 5: take [4,1]
 * - 6: take [4,1,1]
 * - 7: take [4,1,1,1]
 * You can make 8 consecutive integer values starting from 0.
 * Example 3:
 *
 * Input: coins = [1,4,10,3,1]
 * Output: 20
 *  
 * Constraints:
 *
 * 	coins.length == n
 * 	1 <= n <= 4 * 10^4
 * 	1 <= coins[i] <= 4 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-consecutive-values-you-can-make/
// discuss: https://leetcode.com/problems/maximum-number-of-consecutive-values-you-can-make/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();

        let mut total = 0;

        for v in coins {
            if total + 1 < v {
                return total + 1;
            }
            total += v;
        }

        total + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1798_example_1() {
        let coins = vec![1, 3];

        let result = 2;

        assert_eq!(Solution::get_maximum_consecutive(coins), result);
    }

    #[test]
    fn test_1798_example_2() {
        let coins = vec![1, 1, 1, 4];

        let result = 8;

        assert_eq!(Solution::get_maximum_consecutive(coins), result);
    }
}
