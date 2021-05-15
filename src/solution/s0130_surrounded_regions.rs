/**
 * [130] Surrounded Regions
 *
 * Given an m x n matrix board containing 'X' and 'O', capture all regions surrounded by 'X'.
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 550px; height: 237px;" />
 * Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * Explanation: Surrounded regions should not be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.
 *
 * Example 2:
 *
 * Input: board = [["X"]]
 * Output: [["X"]]
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 200
 * 	board[i][j] is 'X' or 'O'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/surrounded-regions/
// discuss: https://leetcode.com/problems/surrounded-regions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if i == 0 || i == board.len() - 1 || j == 0 || j == board[i].len() - 1 {
                    Self::helper(board, i, j);
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == '*' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn helper(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if board[i][j] != 'O' {
            return;
        }

        board[i][j] = '*';
        if i > 0 && board[i - 1][j] == 'O' {
            Self::helper(board, i - 1, j);
        }
        if j < board[i].len() - 1 && board[i][j + 1] == 'O' {
            Self::helper(board, i, j + 1);
        }
        if i < board.len() - 1 && board[i + 1][j] == 'O' {
            Self::helper(board, i + 1, j);
        }
        if j > 1 && board[i][j - 1] == 'O' {
            Self::helper(board, i, j - 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0130_example_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let result = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        Solution::solve(&mut board);

        assert_eq!(board, result);
    }

    #[test]
    fn test_0130_example_2() {
        let mut board = vec![vec!['X']];
        let result = vec![vec!['X']];

        Solution::solve(&mut board);

        assert_eq!(board, result);
    }
}
