/**
 * [1020] Number of Enclaves
 *
 * You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.
 * A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.
 * Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves1.jpg" style="width: 333px; height: 333px;" />
 * Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
 * Output: 3
 * Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves2.jpg" style="width: 333px; height: 333px;" />
 * Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
 * Output: 0
 * Explanation: All 1s are either on the boundary or can reach the boundary.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 500
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-enclaves/
// discuss: https://leetcode.com/problems/number-of-enclaves/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if (i == 0 || j == 0 || i == m - 1 || j == n - 1) && grid[i][j] == 1 {
                    Self::dfs_helper(&mut grid, i, j);
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() {
            return;
        }
        if grid[i][j] != 1 {
            return;
        }
        grid[i][j] = 0;
        Self::dfs_helper(grid, i + 1, j);
        if i >= 1 {
            Self::dfs_helper(grid, i - 1, j);
        }
        Self::dfs_helper(grid, i, j + 1);
        if j >= 1 {
            Self::dfs_helper(grid, i, j - 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1020_example_1() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let result = 3;

        assert_eq!(Solution::num_enclaves(grid), result);
    }

    #[test]
    fn test_1020_example_2() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let result = 0;

        assert_eq!(Solution::num_enclaves(grid), result);
    }
}
