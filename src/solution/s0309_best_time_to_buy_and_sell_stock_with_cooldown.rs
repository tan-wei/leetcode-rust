/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
 *
 * 	After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
 *
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 *
 * Input: prices = [1,2,3,0,2]
 * Output: 3
 * Explanation: transactions = [buy, sell, cooldown, buy, sell]
 *
 * Example 2:
 *
 * Input: prices = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 5000
 * 	0 <= prices[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n <= 1 {
            return 0;
        }
        // the max profit end with buy
        let mut buy = vec![0; n];

        // the max profit end with sell
        let mut sell = vec![0; n];

        // the max profit end with cooldown
        let mut rest = vec![0; n];

        buy[0] = -prices[0];
        sell[0] = 0;
        rest[0] = 0;

        for i in 1..n {
            buy[i] = std::cmp::max(rest[i - 1] - prices[i], buy[i - 1]);
            sell[i] = std::cmp::max(buy[i - 1] + prices[i], sell[i - 1]);
            rest[i] = std::cmp::max(sell[i - 1], rest[i - 1]);
        }

        std::cmp::max(
            *buy.last().unwrap(),
            std::cmp::max(*sell.last().unwrap(), *rest.last().unwrap()),
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0309_example_1() {
        let prices = vec![1, 2, 3, 0, 2];
        let result = 3;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0309_example_2() {
        let prices = vec![1];
        let result = 0;

        assert_eq!(Solution::max_profit(prices), result);
    }
}
