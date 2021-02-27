/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [["Q"]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/n-queens/discuss/774044/Backtracking-in-Rust-100-in-both-speed-and-memory-usage
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn queen_helper(
            n: i32,
            row: usize,
            mut candidate: &mut Vec<Vec<u8>>,
            mut queens: &mut Vec<Vec<String>>,
        ) {
            if row == (n as usize) {
                let candidate_str: Vec<String> = candidate
                    .iter()
                    .map(|byte_vec| String::from_utf8(byte_vec.to_vec()).unwrap())
                    .collect();
                queens.push(candidate_str);
                return;
            }
            for col in 0..n {
                let col = col as usize;
                if !is_valid_position(row, col, &candidate) {
                    continue;
                }
                candidate[row][col] = 'Q' as u8;
                queen_helper(n, row + 1, &mut candidate, &mut queens);
                candidate[row][col] = '.' as u8;
            }
        }
        fn is_valid_position(row: usize, col: usize, candidate: &Vec<Vec<u8>>) -> bool {
            let n = candidate.len() as i32;
            let row = row as i32;
            let col = col as i32;
            let same_col_iter = (0..row).zip(std::iter::repeat(col));
            let top_left_iter = (0..row)
                .rev()
                .zip((0..col).rev())
                .take_while(|(i, j)| (i >= &0) && (j >= &0));
            let top_right_iter = (0..row)
                .rev()
                .zip(col + 1..n)
                .take_while(|(i, j)| (i >= &0) && (j < &n));

            !same_col_iter
                .chain(top_left_iter)
                .chain(top_right_iter)
                .any(|(i, j)| candidate[i as usize][j as usize] == ('Q' as u8))
        }

        let mut queens: Vec<Vec<String>> = Vec::with_capacity(n as usize);
        let mut candidate: Vec<Vec<u8>> = vec![".".repeat(n as usize).into_bytes(); n as usize];
        queen_helper(n, 0, &mut candidate, &mut queens);
        return queens;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0051_example_1() {
        let n = 4;
        let result = vec![
            vec_string![".Q..", "...Q", "Q...", "..Q."],
            vec_string!["..Q.", "Q...", "...Q", ".Q.."],
        ];

        assert_eq!(Solution::solve_n_queens(n), result);
    }

    #[test]
    fn test_0051_example_2() {
        let n = 1;
        let result = vec![vec_string!["Q"]];

        assert_eq!(Solution::solve_n_queens(n), result);
    }
}
