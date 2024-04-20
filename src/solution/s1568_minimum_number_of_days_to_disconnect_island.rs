/**
 * [1568] Minimum Number of Days to Disconnect Island
 *
 * You are given an m x n binary grid grid where 1 represents land and 0 represents water. An island is a maximal 4-directionally (horizontal or vertical) connected group of 1's.
 * The grid is said to be connected if we have exactly one island, otherwise is said disconnected.
 * In one day, we are allowed to change any single land cell (1) into a water cell (0).
 * Return the minimum number of days to disconnect the grid.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/land1.jpg" style="width: 500px; height: 169px;" />
 * Input: grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]
 * Output: 2
 * Explanation: We need at least 2 days to get a disconnected grid.
 * Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/land2.jpg" style="width: 404px; height: 85px;" />
 * Input: grid = [[1,1]]
 * Output: 2
 * Explanation: Grid of full water is also disconnected ([[1,1]] -> [[0,0]]), 0 islands.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 30
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/
// discuss: https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/solutions/3171061/just-a-runnable-solution/
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut islands = Self::count_islands(&grid);
        let (n, m) = (grid.len(), grid[0].len());
        if islands > 1 || islands == 0 {
            return 0;
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    islands = Self::count_islands(&grid);
                    grid[i][j] = 1;
                    if islands > 1 || islands == 0 {
                        return 1;
                    }
                }
            }
        }

        2
    }

    fn dfs_helper(x: usize, y: usize, grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<i32>>) {
        let dx = vec![1, -1, 0, 0];
        let dy = vec![0, 0, 1, -1];
        let (n, m) = (grid.len(), grid[0].len());
        vis[x][y] = 1;
        for a in 0..4 {
            let nx = x as i32 + dx[a];
            let ny = y as i32 + dy[a];
            if nx >= 0
                && ny >= 0
                && nx < n as i32
                && ny < m as i32
                && vis[nx as usize][ny as usize] == 0
                && grid[nx as usize][ny as usize] == 1
            {
                Self::dfs_helper(nx as usize, ny as usize, grid, vis);
            }
        }
    }

    fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
        let mut islands = 0;
        let (n, m) = (grid.len(), grid[0].len());
        let mut vis = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if vis[i][j] == 0 && grid[i][j] == 1 {
                    Self::dfs_helper(i, j, grid, &mut vis);
                    islands += 1;
                }
            }
        }
        islands
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1568_example_1() {
        let grid = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];

        let result = 2;

        assert_eq!(Solution::min_days(grid), result);
    }

    #[test]
    fn test_1568_example_2() {
        let grid = vec![vec![1, 1]];

        let result = 2;

        assert_eq!(Solution::min_days(grid), result);
    }
}
