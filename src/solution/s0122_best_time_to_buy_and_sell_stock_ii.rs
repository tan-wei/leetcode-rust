/**
 * [122] Best Time to Buy and Sell Stock II
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 *
 * Input: prices = [7,1,5,3,6,4]
 * Output: 7
 * Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
 * Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
 *
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
 * Explanation: In this case, no transaction is done, i.e., max profit = 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 3 * 10^4
 * 	0 <= prices[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, None), |(result, last), price| {
                (
                    result + price - last.map(|v| price.min(v)).unwrap_or(price),
                    Some(price),
                )
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0122_example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = 7;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0122_example_2() {
        let prices = vec![1, 2, 3, 4, 5];
        let result = 4;

        assert_eq!(Solution::max_profit(prices), result);
    }

    #[test]
    fn test_0122_example_3() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = 0;

        assert_eq!(Solution::max_profit(prices), result);
    }
}
