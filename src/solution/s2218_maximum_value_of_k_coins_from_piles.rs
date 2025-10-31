/**
 * [2218] Maximum Value of K Coins From Piles
 *
 * There are n piles of coins on a table. Each pile consists of a positive number of coins of assorted denominations.
 * In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet.
 * Given a list piles, where piles[i] is a list of integers denoting the composition of the i^th pile from top to bottom, and a positive integer k, return the maximum total value of coins you can have in your wallet if you choose exactly k coins optimally.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/09/e1.png" style="width: 600px; height: 243px;" />
 * Input: piles = [[1,100,3],[7,8,9]], k = 2
 * Output: 101
 * Explanation:
 * The above diagram shows the different ways we can choose k coins.
 * The maximum total we can obtain is 101.
 *
 * Example 2:
 *
 * Input: piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
 * Output: 706
 * Explanation:
 * The maximum total can be obtained if we choose all coins from the last pile.
 *
 *  
 * Constraints:
 *
 * 	n == piles.length
 * 	1 <= n <= 1000
 * 	1 <= piles[i][j] <= 10^5
 * 	1 <= k <= sum(piles[i].length) <= 2000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/
// discuss: https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2218_example_1() {
        let piles = vec![vec![1, 100, 3], vec![7, 8, 9]];
        let k = 2;

        let result = 101;

        assert_eq!(Solution::max_value_of_coins(piles, k), result);
    }

    #[test]
    #[ignore]
    fn test_2218_example_2() {
        let piles = vec![
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![1, 1, 1, 1, 1, 1, 700],
        ];
        let k = 7;

        let result = 706;

        assert_eq!(Solution::max_value_of_coins(piles, k), result);
    }
}
