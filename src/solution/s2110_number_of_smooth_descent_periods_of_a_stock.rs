/**
 * [2110] Number of Smooth Descent Periods of a Stock
 *
 * You are given an integer array prices representing the daily price history of a stock, where prices[i] is the stock price on the i^th day.
 * A smooth descent period of a stock consists of one or more contiguous days such that the price on each day is lower than the price on the preceding day by exactly 1. The first day of the period is exempted from this rule.
 * Return the number of smooth descent periods.
 *  
 * Example 1:
 *
 * Input: prices = [3,2,1,4]
 * Output: 7
 * Explanation: There are 7 smooth descent periods:
 * [3], [2], [1], [4], [3,2], [2,1], and [3,2,1]
 * Note that a period with one day is a smooth descent period by the definition.
 *
 * Example 2:
 *
 * Input: prices = [8,6,7,7]
 * Output: 4
 * Explanation: There are 4 smooth descent periods: [8], [6], [7], and [7]
 * Note that [8,6] is not a smooth descent period as 8 - 6 &ne; 1.
 *
 * Example 3:
 *
 * Input: prices = [1]
 * Output: 1
 * Explanation: There is 1 smooth descent period: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 10^5
 * 	1 <= prices[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/
// discuss: https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2110_example_1() {
        let prices = vec![3, 2, 1, 4];

        let result = 7;

        assert_eq!(Solution::get_descent_periods(prices), result);
    }

    #[test]
    #[ignore]
    fn test_2110_example_2() {
        let prices = vec![8, 6, 7, 7];

        let result = 4;

        assert_eq!(Solution::get_descent_periods(prices), result);
    }

    #[test]
    #[ignore]
    fn test_2110_example_3() {
        let prices = vec![1];

        let result = 4;

        assert_eq!(Solution::get_descent_periods(prices), result);
    }
}
