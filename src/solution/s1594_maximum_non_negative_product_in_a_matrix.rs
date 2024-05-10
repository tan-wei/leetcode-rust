/**
 * [1594] Maximum Non Negative Product in a Matrix
 *
 * You are given a m x n matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.
 * Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (m - 1, n - 1), find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.
 * Return the maximum non-negative product modulo 10^9 + 7. If the maximum product is negative, return -1.
 * Notice that the modulo is performed after getting the maximum product.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product1.jpg" style="width: 244px; height: 245px;" />
 * Input: grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
 * Output: -1
 * Explanation: It is not possible to get non-negative product in the path from (0, 0) to (2, 2), so return -1.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product2.jpg" style="width: 244px; height: 245px;" />
 * Input: grid = [[1,-2,1],[1,-2,1],[3,-4,1]]
 * Output: 8
 * Explanation: Maximum non-negative product is shown (1 * 1 * -2 * -4 * 1 = 8).
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product3.jpg" style="width: 164px; height: 165px;" />
 * Input: grid = [[1,3],[0,-4]]
 * Output: 0
 * Explanation: Maximum non-negative product is shown (1 * 0 * -4 = 0).
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 15
 * 	-4 <= grid[i][j] <= 4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/
// discuss: https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/solutions/1184944/rust-dp-solution-1-d-vector/
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());
        let mut dp = Vec::with_capacity(c);

        dp.push(Self::make_new_range((1, 1), grid[0][0] as i64));

        for i in 1..c {
            dp.push(Self::make_new_range(dp[i - 1], grid[0][i] as i64));
        }

        for i in 1..r {
            dp[0] = Self::make_new_range(dp[0], grid[i][0] as i64);

            for j in 1..c {
                let new_range1 = Self::make_new_range(dp[j], grid[i][j] as i64);
                let new_range2 = Self::make_new_range(dp[j - 1], grid[i][j] as i64);

                dp[j] = (
                    new_range1.0.min(new_range2.0),
                    new_range1.1.max(new_range2.1),
                );
            }
        }

        let max = dp[c - 1].1;
        if max < 0 {
            return -1;
        }

        (max % MOD) as i32
    }

    fn make_new_range(old_range: (i64, i64), value: i64) -> (i64, i64) {
        if value > 0 {
            (old_range.0 * value, old_range.1 * value)
        } else {
            (old_range.1 * value, old_range.0 * value)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1594_example_1() {
        let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];

        let result = -1;

        assert_eq!(Solution::max_product_path(grid), result);
    }

    #[test]
    fn test_1594_example_2() {
        let grid = vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]];

        let result = 8;

        assert_eq!(Solution::max_product_path(grid), result);
    }

    #[test]
    fn test_1594_example_3() {
        let grid = vec![vec![1, 3], vec![0, -4]];

        let result = 0;

        assert_eq!(Solution::max_product_path(grid), result);
    }
}
