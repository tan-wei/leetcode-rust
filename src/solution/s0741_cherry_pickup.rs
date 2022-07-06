/**
 * [0741] Cherry Pickup
 *
 * You are given an n x n grid representing a field of cherries, each cell is one of three possible integers.
 *
 * 	0 means the cell is empty, so you can pass through,
 * 	1 means the cell contains a cherry that you can pick up and pass through, or
 * 	-1 means the cell contains a thorn that blocks your way.
 *
 * Return the maximum number of cherries you can collect by following the rules below:
 *
 * 	Starting at the position (0, 0) and reaching (n - 1, n - 1) by moving right or down through valid path cells (cells with value 0 or 1).
 * 	After reaching (n - 1, n - 1), returning to (0, 0) by moving left or up through valid path cells.
 * 	When passing through a path cell containing a cherry, you pick it up, and the cell becomes an empty cell 0.
 * 	If there is no valid path between (0, 0) and (n - 1, n - 1), then no cherries can be collected.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/grid.jpg" style="width: 242px; height: 242px;" />
 * Input: grid = [[0,1,-1],[1,0,-1],[1,1,1]]
 * Output: 5
 * Explanation: The player started at (0, 0) and went down, down, right right to reach (2, 2).
 * 4 cherries were picked up during this single trip, and the matrix becomes [[0,1,-1],[0,0,-1],[0,0,0]].
 * Then, the player went left, up, up, left to return home, picking up one more cherry.
 * The total number of cherries picked up is 5, and this is the maximum possible.
 *
 * Example 2:
 *
 * Input: grid = [[1,1,-1],[1,-1,1],[-1,1,1]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 50
 * 	grid[i][j] is -1, 0, or 1.
 * 	grid[0][0] != -1
 * 	grid[n - 1][n - 1] != -1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cherry-pickup/
// discuss: https://leetcode.com/problems/cherry-pickup/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/cherry-pickup/discuss/817454/Rust-translated-0ms-100
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![std::i32::MIN; n]; n];
        dp[0][0] = grid[0][0];

        for t in 1..2 * n - 1 {
            let mut dp2 = vec![vec![std::i32::MIN; n]; n];
            let (r1, r2) = if n - 1 < t {
                (t + 1 - n, n - 1)
            } else {
                (0, t)
            };
            for i in r1..=r2 {
                for j in r1..=r2 {
                    if grid[i][t - i] == -1 || grid[j][t - j] == -1 {
                        continue;
                    }
                    let mut val = grid[i][t - i];
                    if i != j {
                        val += grid[j][t - j];
                    }
                    for pi in i as i32 - 1..=i as i32 {
                        for pj in j as i32 - 1..=j as i32 {
                            if pi >= 0 && pj >= 0 {
                                dp2[i][j] =
                                    std::cmp::max(dp2[i][j], dp[pi as usize][pj as usize] + val);
                            }
                        }
                    }
                }
            }
            dp = dp2;
        }
        std::cmp::max(0, dp[n - 1][n - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0741_example_1() {
        let grid = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        let result = 5;

        assert_eq!(Solution::cherry_pickup(grid), result);
    }

    #[test]
    fn test_0741_example_2() {
        let grid = vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]];
        let result = 0;

        assert_eq!(Solution::cherry_pickup(grid), result);
    }
}
