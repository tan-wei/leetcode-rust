/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete at most two transactions.
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 *
 * Input: prices = [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 * Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
 * Example 2:
 *
 * Input: prices = [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 * Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
 *
 * Example 3:
 *
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 *
 * Example 4:
 *
 * Input: prices = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 10^5
 * 	0 <= prices[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sell1 = 0;
        let mut sell2 = 0;
        let mut buy1 = i32::MIN;
        let mut buy2 = i32::MIN;

        for i in (0..prices.len()) {
            buy1 = buy1.max(-prices[i]);
            sell1 = sell1.max(buy1 + prices[i]);
            buy2 = buy2.max(sell1 - prices[i]);
            sell2 = sell2.max(buy2 + prices[i]);
        }

        sell2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0123_example_1() {
        let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
        let result = 6;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0123_example_2() {
        let prices = vec![1, 2, 3, 4, 5];
        let result = 4;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0123_example_3() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = 0;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0123_example_4() {
        let prices = vec![1];
        let result = 0;

        assert_eq!(Solution::max_profit(prices), result);
    }
}
