/**
 * [0794] Valid Tic-Tac-Toe State
 *
 * Given a Tic-Tac-Toe board as a string array board, return true if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.
 * The board is a 3 x 3 array that consists of characters ' ', 'X', and 'O'. The ' ' character represents an empty square.
 * Here are the rules of Tic-Tac-Toe:
 *
 * 	Players take turns placing characters into empty squares ' '.
 * 	The first player always places 'X' characters, while the second player always places 'O' characters.
 * 	'X' and 'O' characters are always placed into empty squares, never filled ones.
 * 	The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
 * 	The game also ends if all squares are non-empty.
 * 	No more moves can be played if the game is over.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["O  ","   ","   "]
 * Output: false
 * Explanation: The first player always plays "X".
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe2-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["XOX"," X ","   "]
 * Output: false
 * Explanation: Players take turns making moves.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe4-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["XOX","O O","XOX"]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	board.length == 3
 * 	board[i].length == 3
 * 	board[i][j] is either 'X', 'O', or ' '.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-tic-tac-toe-state/
// discuss: https://leetcode.com/problems/valid-tic-tac-toe-state/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/valid-tic-tac-toe-state/discuss/2207012/Rust-Straightforward-solution
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut sums = [
            0, 0, 0, // row_sums
            0, 0, 0, // col_sums
            0, 0,
        ];

        let (row_sums, more) = sums.split_at_mut(3);
        let (col_sums, dia_sums) = more.split_at_mut(3);

        let mut count = 0;

        macro_rules! update {
            ($row:expr_2021, $col:expr_2021, $incr:expr_2021) => {
                count += $incr;
                row_sums[$row] += $incr;
                col_sums[$col] += $incr;
                if $row == $col {
                    dia_sums[0] += $incr;
                }
                if $row == 2 - $col {
                    dia_sums[1] += $incr;
                }
            };
        }

        for r in 0..3 {
            for c in 0..3 {
                match board[r].as_bytes()[c] {
                    b'X' => {
                        update!(r, c, 1);
                    }
                    b'O' => {
                        update!(r, c, -1);
                    }
                    _ => {}
                }
            }
        }

        if count == 0 || count == 1 {
            let mut x_wins = 0;
            let mut o_wins = 0;

            for n in sums {
                match n {
                    3 => {
                        x_wins += 1;
                    }
                    -3 => {
                        o_wins += 1;
                    }
                    _ => {}
                }
            }
            count == 1 && x_wins >= 0 && o_wins == 0 || count == 0 && x_wins == 0 && o_wins >= 0
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0794_example_1() {
        let board = vec_string!["O  ", "   ", "   "];
        let result = false;

        assert_eq!(Solution::valid_tic_tac_toe(board), result);
    }

    #[test]
    fn test_0794_example_2() {
        let board = vec_string!["XOX", " X ", "   "];
        let result = false;

        assert_eq!(Solution::valid_tic_tac_toe(board), result);
    }

    #[test]
    fn test_0794_example_3() {
        let board = vec_string!["XOX", "O O", "XOX"];
        let result = true;

        assert_eq!(Solution::valid_tic_tac_toe(board), result);
    }
}
