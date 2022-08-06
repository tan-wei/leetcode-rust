/**
 * [0782] Transform to Chessboard
 *
 * You are given an n x n binary grid board. In each move, you can swap any two rows with each other, or any two columns with each other.
 * Return the minimum number of moves to transform the board into a chessboard board. If the task is impossible, return -1.
 * A chessboard board is a board where no 0's and no 1's are 4-directionally adjacent.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard1-grid.jpg" style="width: 500px; height: 145px;" />
 * Input: board = [[0,1,1,0],[0,1,1,0],[1,0,0,1],[1,0,0,1]]
 * Output: 2
 * Explanation: One potential sequence of moves is shown.
 * The first move swaps the first and second column.
 * The second move swaps the second and third row.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard2-grid.jpg" style="width: 164px; height: 165px;" />
 * Input: board = [[0,1],[1,0]]
 * Output: 0
 * Explanation: Also note that the board with 0 in the top left corner, is also a valid chessboard.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/chessboard3-grid.jpg" style="width: 164px; height: 165px;" />
 * Input: board = [[1,0],[1,0]]
 * Output: -1
 * Explanation: No matter what sequence of moves you make, you cannot end with a valid chessboard.
 *
 *  
 * Constraints:
 *
 * 	n == board.length
 * 	n == board[i].length
 * 	2 <= n <= 30
 * 	board[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/transform-to-chessboard/
// discuss: https://leetcode.com/problems/transform-to-chessboard/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/transform-to-chessboard/discuss/1487590/Rust-translated
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        for i in 1..n {
            if (1..n as i32).contains(&(0..n).map(|j| board[i][j] ^ board[0][j]).sum::<i32>()) {
                return -1;
            }
            if (1..n as i32).contains(&(0..n).map(|j| board[j][i] ^ board[j][0]).sum::<i32>()) {
                return -1;
            }
        }

        if ((0..n).map(|i| board[0][i]).sum::<i32>() * 2 - n as i32).abs() > 1 {
            return -1;
        }

        if ((0..n).map(|i| board[i][0]).sum::<i32>() * 2 - n as i32).abs() > 1 {
            return -1;
        }

        let mut rowdiff = (0..n).filter(|&i| board[0][i] == i as i32 % 2).count();
        if rowdiff % 2 != 0 || (n % 2 == 0 && rowdiff * 2 > n) {
            rowdiff = n - rowdiff;
        }

        let mut coldiff = (0..n).filter(|&i| board[i][0] == i as i32 % 2).count();
        if coldiff % 2 != 0 || (n % 2 == 0 && coldiff * 2 > n) {
            coldiff = n - coldiff;
        }

        (rowdiff + coldiff) as i32 / 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0782_example_1() {
        let board = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
        ];
        let result = 2;

        assert_eq!(Solution::moves_to_chessboard(board), result);
    }

    #[test]
    fn test_0782_example_2() {
        let board = vec![vec![0, 1], vec![1, 0]];
        let result = 0;

        assert_eq!(Solution::moves_to_chessboard(board), result);
    }

    #[test]
    fn test_0782_example_3() {
        let board = vec![vec![1, 0], vec![1, 0]];
        let result = -1;

        assert_eq!(Solution::moves_to_chessboard(board), result);
    }
}
