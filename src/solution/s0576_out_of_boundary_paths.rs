/**
 * [0576] Out of Boundary Paths
 *
 * There is an m x n grid with a ball. The ball is initially at the position [startRow, startColumn]. You are allowed to move the ball to one of the four adjacent cells in the grid (possibly out of the grid crossing the grid boundary). You can apply at most maxMove moves to the ball.
 * Given the five integers m, n, maxMove, startRow, startColumn, return the number of paths to move the ball out of the grid boundary. Since the answer can be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_1.png" style="width: 500px; height: 296px;" />
 * Input: m = 2, n = 2, maxMove = 2, startRow = 0, startColumn = 0
 * Output: 6
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_2.png" style="width: 500px; height: 293px;" />
 * Input: m = 1, n = 3, maxMove = 3, startRow = 0, startColumn = 1
 * Output: 12
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 50
 * 	0 <= maxMove <= 50
 * 	0 <= startRow < m
 * 	0 <= startColumn < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/out-of-boundary-paths/
// discuss: https://leetcode.com/problems/out-of-boundary-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i32 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/out-of-boundary-paths/discuss/1294460/Rust-DP-solution
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        dp[start_row as usize][start_column as usize] = 1;
        let mut result = 0;
        for _ in 0..max_move {
            let mut tmp = vec![vec![0; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if i == 0 {
                        result = (result + dp[i][j]) % MOD;
                    }
                    if i == m - 1 {
                        result = (result + dp[i][j]) % MOD;
                    }
                    if j == 0 {
                        result = (result + dp[i][j]) % MOD;
                    }
                    if j == n - 1 {
                        result = (result + dp[i][j]) % MOD;
                    }
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let row = i.wrapping_add(d[0]);
                        let col = j.wrapping_add(d[1]);
                        if (0..m).contains(&row) && (0..n).contains(&col) {
                            tmp[row][col] = (tmp[row][col] + dp[i][j]) % MOD;
                        }
                    }
                }
            }
            dp = tmp;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0576_example_1() {
        let m = 2;
        let n = 2;
        let max_move = 2;
        let start_row = 0;
        let start_column = 0;

        let result = 6;

        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            result
        );
    }

    #[test]
    fn test_0576_example_2() {
        let m = 1;
        let n = 3;
        let max_move = 3;
        let start_row = 0;
        let start_column = 1;

        let result = 12;

        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            result
        );
    }
}
