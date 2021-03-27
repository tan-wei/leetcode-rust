/**
 * [79] Word Search
 *
 * Given an m x n grid of characters board and a string word, return true if word exists in the grid.
 * The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
 * Output: true
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/word3.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n = board[i].length
 * 	1 <= m, n <= 6
 * 	1 <= word.length <= 15
 * 	board and word consists of only lowercase and uppercase English letters.
 *
 *  
 * Follow up: Could you use search pruning to make your solution faster with a larger board?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search/
// discuss: https://leetcode.com/problems/word-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs_helper(board: &mut Vec<Vec<char>>, word: &str, x: i64, y: i64) -> bool {
            if word.len() == 0 {
                return true;
            }
            if x < 0
                || y < 0
                || x >= board.len() as i64
                || y >= board[0].len() as i64
                || board[x as usize][y as usize] == 0 as char
            {
                return false;
            }
            let n = word.chars().next();
            match n {
                Some(c) => {
                    if c != board[x as usize][y as usize] {
                        return false;
                    }

                    board[x as usize][y as usize] = 0 as char;
                    let remaining = &word[1..];
                    if dfs_helper(board, remaining, x + 1, y)
                        || dfs_helper(board, remaining, x - 1, y)
                        || dfs_helper(board, remaining, x, y - 1)
                        || dfs_helper(board, remaining, x, y + 1)
                    {
                        return true;
                    }

                    board[x as usize][y as usize] = c;

                    false
                }
                None => true,
            }
        }

        let mut board = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if dfs_helper(&mut board, word.as_str(), i as i64, j as i64) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0079_example_1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let result = true;

        assert_eq!(Solution::exist(board, word), result);
    }

    #[test]
    fn test_0079_example_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        let result = true;

        assert_eq!(Solution::exist(board, word), result);
    }

    #[test]
    fn test_0079_example_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        let result = false;

        assert_eq!(Solution::exist(board, word), result);
    }
}
