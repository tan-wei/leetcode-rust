/**
 * [0419] Battleships in a Board
 *
 * Given an m x n matrix board where each cell is a battleship 'X' or empty '.', return the number of the battleships on board.
 * Battleships can only be placed horizontally or vertically on board. In other words, they can only be made of the shape 1 x k (1 row, k columns) or k x 1 (k rows, 1 column), where k can be of any size. At least one horizontal or vertical cell separates between two battleships (i.e., there are no adjacent battleships).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/10/battelship-grid.jpg" style="width: 333px; height: 333px;" />
 * Input: board = [["X",".",".","X"],[".",".",".","X"],[".",".",".","X"]]
 * Output: 2
 *
 * Example 2:
 *
 * Input: board = [["."]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 200
 * 	board[i][j] is either '.' or 'X'.
 *
 *  
 * Follow up: Could you do it in one-pass, using only O(1) extra memory and without modifying the values board?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/battleships-in-a-board/
// discuss: https://leetcode.com/problems/battleships-in-a-board/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.len() == 0 || board[0].len() == 0 {
            return 0;
        }

        let (m, n) = (board.len(), board[0].len());

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X'
                    && (i == 0 || board[i - 1][j] != 'X')
                    && (j == 0 || board[i][j - 1] != 'X')
                {
                    result += 1;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0419_example_1() {
        let board = vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
        ];
        let result = 2;

        assert_eq!(Solution::count_battleships(board), result);
    }

    #[test]
    fn test_0419_example_2() {
        let board = vec![vec!['.']];
        let result = 0;

        assert_eq!(Solution::count_battleships(board), result);
    }
}
