/**
 * [2280] Minimum Lines to Represent a Line Chart
 *
 * You are given a 2D integer array stockPrices where stockPrices[i] = [dayi, pricei] indicates the price of the stock on day dayi is pricei. A line chart is created from the array by plotting the points on an XY plane with the X-axis representing the day and the Y-axis representing the price and connecting adjacent points. One such example is shown below:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/30/1920px-pushkin_population_historysvg.png" style="width: 500px; height: 313px;" />
 * Return the minimum number of lines needed to represent the line chart.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/30/ex0.png" style="width: 400px; height: 400px;" />
 * Input: stockPrices = [[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]
 * Output: 3
 * Explanation:
 * The diagram above represents the input, with the X-axis representing the day and Y-axis representing the price.
 * The following 3 lines can be drawn to represent the line chart:
 * - Line 1 (in red) from (1,7) to (4,4) passing through (1,7), (2,6), (3,5), and (4,4).
 * - Line 2 (in blue) from (4,4) to (5,4).
 * - Line 3 (in green) from (5,4) to (8,1) passing through (5,4), (6,3), (7,2), and (8,1).
 * It can be shown that it is not possible to represent the line chart using less than 3 lines.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/30/ex1.png" style="width: 325px; height: 325px;" />
 * Input: stockPrices = [[3,4],[1,2],[7,8],[2,3]]
 * Output: 1
 * Explanation:
 * As shown in the diagram above, the line chart can be represented with a single line.
 *
 *  
 * Constraints:
 *
 * 	1 <= stockPrices.length <= 10^5
 * 	stockPrices[i].length == 2
 * 	1 <= dayi, pricei <= 10^9
 * 	All dayi are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/
// discuss: https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2280_example_1() {
        let stock_prices = vec![
            vec![1, 7],
            vec![2, 6],
            vec![3, 5],
            vec![4, 4],
            vec![5, 4],
            vec![6, 3],
            vec![7, 2],
            vec![8, 1],
        ];

        let result = 3;

        assert_eq!(Solution::minimum_lines(stock_prices), result);
    }

    #[test]
    #[ignore]
    fn test_2280_example_2() {
        let stock_prices = vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]];

        let result = 1;

        assert_eq!(Solution::minimum_lines(stock_prices), result);
    }
}
