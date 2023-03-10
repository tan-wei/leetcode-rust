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
        let mut rook = (0, 0);
        let mut pawns = vec![];
        let mut bishops = vec![];
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut count = 0;

        (0..8usize).for_each(|i| {
            (0..8usize).for_each(|j| match board[i][j] {
                'R' => rook = (i, j),
                'p' => pawns.push((i, j)),
                'B' => bishops.push((i, j)),
                _ => (),
            })
        });

        pawns = pawns
            .into_iter()
            .filter(|&(i, j)| rook.0 == i || rook.1 == j)
            .collect();

        bishops = bishops
            .into_iter()
            .filter(|&(i, j)| rook.0 == i || rook.1 == j)
            .collect();

        for (x, y) in directions {
            let (mut i, mut j) = rook;
            while (0..8).contains(&i) && (0..8).contains(&j) {
                i += x as usize;
                j += y as usize;
                if bishops.contains(&(i, j)) {
                    break;
                }
                if pawns.contains(&(i, j)) {
                    count += 1;
                    break;
                }
            }
        }
        count
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
