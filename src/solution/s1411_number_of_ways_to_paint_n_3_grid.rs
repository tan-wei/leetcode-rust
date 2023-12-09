/**
 * [1411] Number of Ways to Paint N × 3 Grid
 *
 * You have a grid of size n x 3 and you want to paint each cell of the grid with exactly one of the three colors: Red, Yellow, or Green while making sure that no two adjacent cells have the same color (i.e., no two cells that share vertical or horizontal sides have the same color).
 * Given n the number of rows of the grid, return the number of ways you can paint this grid. As the answer may grow large, the answer must be computed modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/26/e1.png" style="width: 400px; height: 257px;" />
 * Input: n = 1
 * Output: 12
 * Explanation: There are 12 possible way to paint the grid as shown.
 *
 * Example 2:
 *
 * Input: n = 5000
 * Output: 30228214
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	1 <= n <= 5000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
// discuss: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut dp = vec![(6_i64, 6_i64)];

        for _ in 1..n {
            let (a, b) = dp.last().unwrap();
            dp.push(((3 * a + 2 * b) % 1000000007, (2 * a + 2 * b) % 1000000007));
        }

        ((dp.last().unwrap().0 + dp.last().unwrap().1) % 1000000007) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1411_example_1() {
        let n = 1;

        let result = 12;

        assert_eq!(Solution::num_of_ways(n), result);
    }

    #[test]
    fn test_1411_example_2() {
        let n = 5000;

        let result = 30228214;

        assert_eq!(Solution::num_of_ways(n), result);
    }
}
