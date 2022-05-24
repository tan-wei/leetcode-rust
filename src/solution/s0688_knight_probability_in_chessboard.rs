/**
 * [0688] Knight Probability in Chessboard
 *
 * On an n x n chessboard, a knight starts at the cell (row, column) and attempts to make exactly k moves. The rows and columns are 0-indexed, so the top-left cell is (0, 0), and the bottom-right cell is (n - 1, n - 1).
 * A chess knight has eight possible moves it can make, as illustrated below. Each move is two cells in a cardinal direction, then one cell in an orthogonal direction.
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/knight.png" style="width: 300px; height: 300px;" />
 * Each time the knight is to move, it chooses one of eight possible moves uniformly at random (even if the piece would go off the chessboard) and moves there.
 * The knight continues moving until it has made exactly k moves or has moved off the chessboard.
 * Return the probability that the knight remains on the board after it has stopped moving.
 *  
 * Example 1:
 *
 * Input: n =;
 * let k = 2;
 * let row = 0;
 * let column = 0
 * Output: 0.06250
 * Explanation: There are two moves (to (1,2), (2,1)) that will keep the knight on the board.
 * From each of those positions, there are also two moves that will keep the knight on the board.
 * The total probability the knight stays on the board is 0.0625.
 *
 * Example 2:
 *
 * Input: n = 1, k = 0, row = 0, column = 0
 * Output: 1.00000
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 25
 * 	0 <= k <= 100
 * 	0 <= row, column <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/knight-probability-in-chessboard/
// discuss: https://leetcode.com/problems/knight-probability-in-chessboard/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/knight-probability-in-chessboard/discuss/714572/Rust-or-DP
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let dir = [
            (1, -2),
            (1, 2),
            (-1, 2),
            (-1, -2),
            (-2, 1),
            (2, -1),
            (2, 1),
            (-2, -1),
        ];

        let mut dp = vec![vec![0.0; n as usize]; n as usize];
        dp[row as usize][column as usize] = 1.0;

        for _ in 0..k {
            let mut dp1 = vec![vec![0.0; n as usize]; n as usize];
            for (ir, row) in dp.iter().enumerate() {
                let ir = ir as i32;
                for (ic, x) in row.iter().enumerate() {
                    let ic = ic as i32;
                    if *x > 0.0 {
                        for (xd, yd) in &dir {
                            if ir + yd < n && ir + yd >= 0 && ic + xd < n && ic + xd >= 0 {
                                dp1[(ir + yd) as usize][(ic + xd) as usize] += *x / 8.0;
                            }
                        }
                    }
                }
            }
            dp = dp1;
        }

        dp.iter().flatten().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0688_example_1() {
        let n = 3;
        let k = 2;
        let row = 0;
        let column = 0;

        let result = 0.06250;

        assert_f64_near!(Solution::knight_probability(n, k, row, column), result);
    }

    #[test]
    fn test_0688_example_2() {
        let n = 1;
        let k = 0;
        let row = 0;
        let column = 0;

        let result = 1.00000;

        assert_f64_near!(Solution::knight_probability(n, k, row, column), result);
    }
}
