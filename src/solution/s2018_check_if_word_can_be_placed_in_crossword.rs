/**
 * [2018] Check if Word Can Be Placed In Crossword
 *
 * You are given an m x n matrix board, representing the current state of a crossword puzzle. The crossword contains lowercase English letters (from solved words), ' ' to represent any empty cells, and '#' to represent any blocked cells.
 * A word can be placed horizontally (left to right or right to left) or vertically (top to bottom or bottom to top) in the board if:
 *
 * 	It does not occupy a cell containing the character '#'.
 * 	The cell each letter is placed in must either be ' ' (empty) or match the letter already on the board.
 * 	There must not be any empty cells ' ' or other lowercase letters directly left or right of the word if the word was placed horizontally.
 * 	There must not be any empty cells ' ' or other lowercase letters directly above or below the word if the word was placed vertically.
 *
 * Given a string word, return true if word can be placed in board, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/04/crossword-ex1-1.png" style="width: 478px; height: 180px;" />
 * Input: board = [["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]], word = "abc"
 * Output: true
 * Explanation: The word "abc" can be placed as shown above (top to bottom).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/04/crossword-ex2-1.png" style="width: 180px; height: 180px;" />
 * Input: board = [[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]], word = "ac"
 * Output: false
 * Explanation: It is impossible to place the word because there will always be a space/letter above or below it.
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/04/crossword-ex3-1.png" style="width: 478px; height: 180px;" />
 * Input: board = [["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]], word = "ca"
 * Output: true
 * Explanation: The word "ca" can be placed as shown above (right to left).
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m * n <= 2 * 10^5
 * 	board[i][j] will be ' ', '#', or a lowercase English letter.
 * 	1 <= word.length <= max(m, n)
 * 	word will contain only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-word-can-be-placed-in-crossword/
// discuss: https://leetcode.com/problems/check-if-word-can-be-placed-in-crossword/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2018_example_1() {
        let board = vec![
            vec!['#', ' ', '#'],
            vec![' ', ' ', '#'],
            vec!['#', 'c', ' '],
        ];
        let word = "abc".to_string();

        let result = true;

        assert_eq!(Solution::place_word_in_crossword(board, word), result);
    }

    #[test]
    #[ignore]
    fn test_2018_example_2() {
        let board = vec![
            vec![' ', '#', 'a'],
            vec![' ', '#', 'c'],
            vec![' ', '#', 'a'],
        ];
        let word = "ac".to_string();

        let result = false;

        assert_eq!(Solution::place_word_in_crossword(board, word), result);
    }

    #[test]
    #[ignore]
    fn test_2018_example_3() {
        let board = vec![
            vec!['#', ' ', '#'],
            vec![' ', ' ', '#'],
            vec!['#', ' ', 'c'],
        ];
        let word = "ca".to_string();

        let result = true;

        assert_eq!(Solution::place_word_in_crossword(board, word), result);
    }
}
