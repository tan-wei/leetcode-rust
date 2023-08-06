/**
 * [1254] Number of Closed Islands
 *
 * Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group of <font face="monospace">0</font>s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
 * Return the number of closed islands.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_3_1610.png" style="width: 240px; height: 120px;" />
 *
 * Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
 * Output: 2
 * Explanation:
 * Islands in gray are closed because they are completely surrounded by water (group of 1s).
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_4_1610.png" style="width: 160px; height: 80px;" />
 *
 * Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
 * Output: 1
 *
 * Example 3:
 *
 * Input: grid = [[1,1,1,1,1,1,1],
 *                [1,0,0,0,0,0,1],
 *                [1,0,1,1,1,0,1],
 *                [1,0,1,0,1,0,1],
 *                [1,0,1,1,1,0,1],
 *                [1,0,0,0,0,0,1],
 *                [1,1,1,1,1,1,1]]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= grid.length, grid[0].length <= 100
 * 	0 <= grid[i][j] <=1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-closed-islands/
// discuss: https://leetcode.com/problems/number-of-closed-islands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                    Self::dfs_helper(&mut grid, i, j);
                }
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    count += 1;
                    Self::dfs_helper(&mut grid, i, j);
                }
            }
        }
        count
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 1 {
            return;
        }
        grid[i][j] = 1;
        Self::dfs_helper(grid, i + 1, j);
        if i != 0 {
            Self::dfs_helper(grid, i - 1, j);
        }
        Self::dfs_helper(grid, i, j + 1);
        if j != 0 {
            Self::dfs_helper(grid, i, j - 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1254_example_1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        let result = 2;

        assert_eq!(Solution::closed_island(grid), result);
    }

    #[test]
    fn test_1254_example_2() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        let result = 1;

        assert_eq!(Solution::closed_island(grid), result);
    }

    #[test]
    fn test_1254_example_3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        let result = 2;

        assert_eq!(Solution::closed_island(grid), result);
    }
}
