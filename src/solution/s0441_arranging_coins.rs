/**
 * [0441] Arranging Coins
 *
 * You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the i^th row has exactly i coins. The last row of the staircase may be incomplete.
 * Given the integer n, return the number of complete rows of the staircase you will build.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/arrangecoins1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: n = 5
 * Output: 2
 * Explanation: Because the 3^rd row is incomplete, we return 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/arrangecoins2-grid.jpg" style="width: 333px; height: 333px;" />
 * Input: n = 8
 * Output: 3
 * Explanation: Because the 4^th row is incomplete, we return 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arranging-coins/
// discuss: https://leetcode.com/problems/arranging-coins/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (((8.0 * n as f64 + 1.0).sqrt() - 1.0) / 2.0).floor() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0441_example_1() {
        let n = 5;
        let result = 2;

        assert_eq!(Solution::arrange_coins(n), result);
    }

    #[test]
    fn test_0441_example_2() {
        let n = 8;
        let result = 3;

        assert_eq!(Solution::arrange_coins(n), result);
    }
}
