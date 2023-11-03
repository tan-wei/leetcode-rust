/**
 * [1368] Minimum Cost to Make at Least One Valid Path in a Grid
 *
 * Given an m x n grid. Each cell of the grid has a sign pointing to the next cell you should visit if you are currently in this cell. The sign of grid[i][j] can be:
 *
 * 	1 which means go to the cell to the right. (i.e go from grid[i][j] to grid[i][j + 1])
 * 	2 which means go to the cell to the left. (i.e go from grid[i][j] to grid[i][j - 1])
 * 	3 which means go to the lower cell. (i.e go from grid[i][j] to grid[i + 1][j])
 * 	4 which means go to the upper cell. (i.e go from grid[i][j] to grid[i - 1][j])
 *
 * Notice that there could be some signs on the cells of the grid that point outside the grid.
 * You will initially start at the upper left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1) following the signs on the grid. The valid path does not have to be the shortest.
 * You can modify the sign on a cell with cost = 1. You can modify the sign on a cell one time only.
 * Return the minimum cost to make the grid have at least one valid path.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid1.png" style="width: 400px; height: 390px;" />
 * Input: grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
 * Output: 3
 * Explanation: You will start at point (0, 0).
 * The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
 * The total cost = 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid2.png" style="width: 350px; height: 341px;" />
 * Input: grid = [[1,1,3],[3,2,2],[1,1,4]]
 * Output: 0
 * Explanation: You can follow the path from (0, 0) to (2, 2).
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid3.png" style="width: 200px; height: 192px;" />
 * Input: grid = [[1,2],[4,3]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 100
 * 	1 <= grid[i][j] <= 4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
// discuss: https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/solutions/2790856/rust-solution-bfs-with-pq-100-10ms/
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut q) = (
            grid.len(),
            grid[0].len(),
            std::collections::BinaryHeap::from([(0, 0, 0)]),
        );
        let mut dp = [[1000; 101]; 101];
        dp[0][0] = 0;
        while q.len() > 0 {
            let (curr, i, j) = q.pop().unwrap();
            if -curr > dp[i][j] {
                continue;
            }
            if (i, j) == (n - 1, m - 1) {
                return dp[n - 1][m - 1];
            }
            if i + 1 < n {
                let c = curr - (grid[i][j] != 3) as i32;
                if dp[i + 1][j] > -c {
                    dp[i + 1][j] = -c;
                    q.push((c, i + 1, j));
                }
            }
            if j + 1 < m {
                let c = curr - (grid[i][j] != 1) as i32;
                if dp[i][j + 1] > -c {
                    dp[i][j + 1] = -c;
                    q.push((c, i, j + 1));
                }
            }
            if i > 0 {
                let c = curr - (grid[i][j] != 4) as i32;
                if dp[i - 1][j] > -c {
                    dp[i - 1][j] = -c;
                    q.push((c, i - 1, j));
                }
            }
            if j > 0 {
                let c = curr - (grid[i][j] != 2) as i32;
                if dp[i][j - 1] > -c {
                    dp[i][j - 1] = -c;
                    q.push((c, i, j - 1));
                }
            }
        }
        dp[n - 1][m - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1368_example_1() {
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ];

        let result = 3;

        assert_eq!(Solution::min_cost(grid), result);
    }

    #[test]
    fn test_1368_example_2() {
        let grid = vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]];

        let result = 0;

        assert_eq!(Solution::min_cost(grid), result);
    }

    #[test]
    fn test_1368_example_3() {
        let grid = vec![vec![1, 2], vec![4, 3]];

        let result = 1;

        assert_eq!(Solution::min_cost(grid), result);
    }
}
