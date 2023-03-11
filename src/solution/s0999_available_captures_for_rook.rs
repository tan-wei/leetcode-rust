/**
 * [0999] Available Captures for Rook
 *
 * On an 8 x 8 chessboard, there is exactly one white rook 'R' and some number of white bishops 'B', black pawns 'p', and empty squares '.'.
 * When the rook moves, it chooses one of four cardinal directions (north, east, south, or west), then moves in that direction until it chooses to stop, reaches the edge of the board, captures a black pawn, or is blocked by a white bishop. A rook is considered attacking a pawn if the rook can capture the pawn on the rook's turn. The number of available captures for the white rook is the number of pawns that the rook is attacking.
 * Return the number of available captures for the white rook.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/20/1253_example_1_improved.PNG" style="width: 300px; height: 305px;" />
 * Input: board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","R",".",".",".","p"],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
 * Output: 3
 * Explanation: In this example, the rook is attacking all the pawns.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/19/1253_example_2_improved.PNG" style="width: 300px; height: 306px;" />
 * Input: board = [[".",".",".",".",".",".",".","."],[".","p","p","p","p","p",".","."],[".","p","p","B","p","p",".","."],[".","p","B","R","B","p",".","."],[".","p","p","B","p","p",".","."],[".","p","p","p","p","p",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
 * Output: 0
 * Explanation: The bishops are blocking the rook from attacking any of the pawns.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/20/1253_example_3_improved.PNG" style="width: 300px; height: 305px;" />
 * Input: board = [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","p",".",".",".","."],["p","p",".","R",".","p","B","."],[".",".",".",".",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."]]
 * Output: 3
 * Explanation: The rook is attacking the pawns at positions b5, d6, and f5.
 *
 *  
 * Constraints:
 *
 * 	board.length == 8
 * 	board[i].length == 8
 * 	board[i][j] is either 'R', '.', 'B', or 'p'
 * 	There is exactly one cell with board[i][j] == 'R'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/available-captures-for-rook/
// discuss: https://leetcode.com/problems/available-captures-for-rook/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        for (y, row) in board.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == 'R') {
                let row_it = row.iter().cloned();
                let col_it = (0..8_usize).map(|i| board[i][x]);
                return Self::calc_cap(row_it, x) + Self::calc_cap(col_it, y);
            }
        }

        unreachable!()
    }

    fn calc_cap(it: impl Iterator<Item = char>, rook_pos: usize) -> i32 {
        let (mut res, mut num) = (0, 0);
        for (pos, fig) in it.enumerate() {
            match (fig, pos < rook_pos) {
                ('R', _) => res += num,
                ('p', true) => num = 1,
                ('B', true) => num = 0,
                ('p', false) => return res + 1,
                ('B', false) => return res,
                _ => (),
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0999_example_1() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let result = 3;

        assert_eq!(Solution::num_rook_captures(board), result);
    }

    #[test]
    fn test_0999_example_2() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let result = 0;

        assert_eq!(Solution::num_rook_captures(board), result);
    }

    #[test]
    fn test_0999_example_3() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let result = 3;

        assert_eq!(Solution::num_rook_captures(board), result);
    }
}
