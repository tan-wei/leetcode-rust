/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 * An obstacle and space is marked as 1 and 0 respectively in the grid.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot1.jpg" style="width: 242px; height: 242px;" />
 * Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: 2
 * Explanation: There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot2.jpg" style="width: 162px; height: 162px;" />
 * Input: obstacleGrid = [[0,1],[0,0]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	m == obstacleGrid.length
 * 	n == obstacleGrid[i].length
 * 	1 <= m, n <= 100
 * 	obstacleGrid[i][j] is 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths-ii/
// discuss: https://leetcode.com/problems/unique-paths-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut dp = vec![vec![0; n]; m];

        for i in (0..=m - 1).rev() {
            for j in (0..=n - 1).rev() {
                match obstacle_grid[i][j] {
                    1 => dp[i][j] = 0,
                    _ => {
                        if i == m - 1 && j == n - 1 {
                            dp[i][j] = 1;
                        } else if i == m - 1 {
                            dp[i][j] = dp[i][j + 1];
                        } else if j == n - 1 {
                            dp[i][j] = dp[i + 1][j];
                        } else {
                            dp[i][j] = dp[i + 1][j] + dp[i][j + 1];
                        }
                    }
                }
            }
        }

        dp[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0063_example_1() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let result = 2;

        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), result);
    }

    #[test]
    fn test_0063_example_2() {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        let result = 1;

        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), result);
    }
}
