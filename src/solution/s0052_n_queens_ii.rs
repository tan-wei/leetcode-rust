/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
type QueenPositions = Vec<Option<i16>>;

impl Solution {
    // Credit: https://leetcode.com/problems/n-queens-ii/discuss/668640/Rust-backtracking-solution-with-O(n)-memory-and-0ms-runtime.
    pub fn total_n_queens(n: i32) -> i32 {
        let mut queens: QueenPositions = vec![None; n as usize];
        Self::count_queen_configs(&mut queens, 0)
    }

    fn count_queen_configs(queens: &mut QueenPositions, row: usize) -> i32 {
        if row >= queens.len() {
            return 1;
        }

        (0..queens.len())
            .map(|col| {
                if Self::is_invalid_pos(queens, row as i16, col as i16) {
                    return 0;
                }

                queens[row] = Some(col as i16);
                let result = Self::count_queen_configs(queens, row + 1);
                queens[row] = None;

                result
            })
            .sum()
    }

    fn is_invalid_pos(queens: &QueenPositions, row: i16, col: i16) -> bool {
        queens
            .iter()
            .enumerate()
            .filter_map(|(other_row, other_col)| match other_col {
                Some(column) => Some((other_row, column)),
                None => None,
            })
            .any(|(other_row, other_col)| {
                let column_invalid = *other_col == col;
                let main_diagonal_invalid = (col - *other_col) == (row - other_row as i16);
                let off_diagonal_invalid = other_row as i16 == -(*other_col) + (row + col);
                column_invalid || main_diagonal_invalid || off_diagonal_invalid
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0052_example_1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn test_0052_example_2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
