/**
 * [2617] Minimum Number of Visited Cells in a Grid
 *
 * You are given a 0-indexed m x n integer matrix grid. Your initial position is at the top-left cell (0, 0).
 * Starting from the cell (i, j), you can move to one of the following cells:
 *
 * 	Cells (i, k) with j < k <= grid[i][j] + j (rightward movement), or
 * 	Cells (k, j) with i < k <= grid[i][j] + i (downward movement).
 *
 * Return the minimum number of cells you need to visit to reach the bottom-right cell (m - 1, n - 1). If there is no valid path, return -1.
 *  
 * Example 1:
 *
 * Input: grid = [[3,4,2,1],[4,2,3,1],[2,1,0,0],[2,4,0,0]]
 * Output: 4
 * Explanation: The image above shows one of the paths that visits exactly 4 cells.
 *
 * Example 2:
 *
 * Input: grid = [[3,4,2,1],[4,2,1,1],[2,1,1,0],[3,4,1,0]]
 * Output: 3
 * Explanation: The image above shows one of the paths that visits exactly 3 cells.
 *
 * Example 3:
 *
 * Input: grid = [[2,1,0],[1,0,0]]
 * Output: -1
 * Explanation: It can be proven that no path exists.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 10^5
 * 	1 <= m * n <= 10^5
 * 	0 <= grid[i][j] < m * n
 * 	grid[m - 1][n - 1] == 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-visited-cells-in-a-grid/
// discuss: https://leetcode.com/problems/minimum-number-of-visited-cells-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2617_example_1() {
        let grid = vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 3, 1],
            vec![2, 1, 0, 0],
            vec![2, 4, 0, 0],
        ];

        let result = 4;

        assert_eq!(Solution::minimum_visited_cells(grid), result);
    }

    #[test]
    #[ignore]
    fn test_2617_example_2() {
        let grid = vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 1, 1],
            vec![2, 1, 1, 0],
            vec![3, 4, 1, 0],
        ];

        let result = 3;

        assert_eq!(Solution::minimum_visited_cells(grid), result);
    }

    #[test]
    #[ignore]
    fn test_2617_example_3() {
        let grid = vec![vec![2, 1, 0], vec![1, 0, 0]];

        let result = -1;

        assert_eq!(Solution::minimum_visited_cells(grid), result);
    }
}
