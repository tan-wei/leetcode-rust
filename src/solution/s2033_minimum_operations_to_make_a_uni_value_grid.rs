/**
 * [2033] Minimum Operations to Make a Uni-Value Grid
 *
 * You are given a 2D integer grid of size m x n and an integer x. In one operation, you can add x to or subtract x from any element in the grid.
 * A uni-value grid is a grid where all the elements of it are equal.
 * Return the minimum number of operations to make the grid uni-value. If it is not possible, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/21/gridtxt.png" style="width: 164px; height: 165px;" />
 * Input: grid = [[2,4],[6,8]], x = 2
 * Output: 4
 * Explanation: We can make every element equal to 4 by doing the following:
 * - Add x to 2 once.
 * - Subtract x from 6 once.
 * - Subtract x from 8 twice.
 * A total of 4 operations were used.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/21/gridtxt-1.png" style="width: 164px; height: 165px;" />
 * Input: grid = [[1,5],[2,3]], x = 1
 * Output: 5
 * Explanation: We can make every element equal to 3.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/21/gridtxt-2.png" style="width: 164px; height: 165px;" />
 * Input: grid = [[1,2],[3,4]], x = 2
 * Output: -1
 * Explanation: It is impossible to make every element equal.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 10^5
 * 	1 <= m * n <= 10^5
 * 	1 <= x, grid[i][j] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2033_example_1() {
        let grid = vec![vec![2, 4], vec![6, 8]];
        let x = 2;

        let result = 4;

        assert_eq!(Solution::min_operations(grid, x), result);
    }

    #[test]
    #[ignore]
    fn test_2033_example_2() {
        let grid = vec![vec![1, 5], vec![2, 3]];
        let x = 1;

        let result = 5;

        assert_eq!(Solution::min_operations(grid, x), result);
    }

    #[test]
    #[ignore]
    fn test_2033_example_3() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let x = 2;

        let result = -1;

        assert_eq!(Solution::min_operations(grid, x), result);
    }
}
