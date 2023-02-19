/**
 * [0980] Unique Paths III
 *
 * You are given an m x n integer array grid where grid[i][j] could be:
 *
 * 	1 representing the starting square. There is exactly one starting square.
 * 	2 representing the ending square. There is exactly one ending square.
 * 	0 representing empty squares we can walk over.
 * 	-1 representing obstacles that we cannot walk over.
 *
 * Return the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique1.jpg" style="width: 324px; height: 245px;" />
 * Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
 * Output: 2
 * Explanation: We have the following two paths:
 * 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
 * 2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique2.jpg" style="width: 324px; height: 245px;" />
 * Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
 * Output: 4
 * Explanation: We have the following four paths:
 * 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
 * 2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
 * 3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
 * 4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique3-.jpg" style="width: 164px; height: 165px;" />
 * Input: grid = [[0,1],[2,0]]
 * Output: 0
 * Explanation: There is no path that walks over every empty square exactly once.
 * Note that the starting and ending square can be anywhere in the grid.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 20
 * 	1 <= m * n <= 20
 * 	-1 <= grid[i][j] <= 2
 * 	There is exactly one starting cell and one ending cell.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths-iii/
// discuss: https://leetcode.com/problems/unique-paths-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut pos = (0, 0);
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => count += 1,
                    1 => pos = (i, j),
                    _ => {}
                }
            }
        }
        Solution::dfs_helper(&mut grid, pos, count + 1)
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, pos: (usize, usize), count: usize) -> i32 {
        if grid[pos.0][pos.1] == 2 {
            return if count == 0 { 1 } else { 0 };
        }
        grid[pos.0][pos.1] = 1;
        let result = if pos.0 > 0 && grid[pos.0 - 1][pos.1].abs() != 1 {
            Solution::dfs_helper(grid, (pos.0 - 1, pos.1), count - 1)
        } else {
            0
        } + if pos.1 > 0 && grid[pos.0][pos.1 - 1].abs() != 1 {
            Solution::dfs_helper(grid, (pos.0, pos.1 - 1), count - 1)
        } else {
            0
        } + if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1].abs() != 1 {
            Solution::dfs_helper(grid, (pos.0 + 1, pos.1), count - 1)
        } else {
            0
        } + if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1].abs() != 1 {
            Solution::dfs_helper(grid, (pos.0, pos.1 + 1), count - 1)
        } else {
            0
        };
        grid[pos.0][pos.1] = 0;
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0980_example_1() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
        let result = 2;

        assert_eq!(Solution::unique_paths_iii(grid), result);
    }

    #[test]
    fn test_0980_example_2() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        let result = 4;

        assert_eq!(Solution::unique_paths_iii(grid), result);
    }

    #[test]
    fn test_0980_example_3() {
        let grid = vec![vec![0, 1], vec![2, 0]];
        let result = 0;

        assert_eq!(Solution::unique_paths_iii(grid), result);
    }
}
