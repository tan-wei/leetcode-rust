/**
 * [1138] Alphabet Board Path
 *
 * On an alphabet board, we start at position (0, 0), corresponding to character board[0][0].
 *
 * Here, board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"], as shown in the diagram below.
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/07/28/azboard.png" style="width: 250px; height: 317px;" />
 *
 * We may make the following moves:
 *
 *
 * 	'U' moves our position up one row, if the position exists on the board;
 * 	'D' moves our position down one row, if the position exists on the board;
 * 	'L' moves our position left one column, if the position exists on the board;
 * 	'R' moves our position right one column, if the position exists on the board;
 * 	'!' adds the character board[r][c] at our current position (r, c) to the answer.
 *
 *
 * (Here, the only positions that exist on the board are positions with letters on them.)
 *
 * Return a sequence of moves that makes our answer equal to target in the minimum number of moves.  You may return any path that does so.
 *
 *  
 * Example 1:
 * Input: target = "leet"
 * Output: "DDR!UURRR!!DDD!"
 * Example 2:
 * Input: target = "code"
 * Output: "RR!DDRR!UUL!R!"
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= target.length <= 100
 * 	target consists only of English lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/alphabet-board-path/
// discuss: https://leetcode.com/problems/alphabet-board-path/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut start = 'a' as u8;
        let mut result = String::from("");

        for c in target.chars() {
            let dest = c as u8;

            while start != dest {
                let current_row = (start - 'a' as u8) / 5;
                let target_row = (dest - 'a' as u8) / 5;

                if current_row > target_row {
                    result.push('U');
                    start -= 5;
                } else if current_row < target_row {
                    if start + 5 > 122 {
                        result.push('L');
                        start -= 1;
                    } else {
                        result.push('D');
                        start += 5;
                    }
                } else {
                    if start > dest {
                        result.push('L');
                        start -= 1;
                    } else {
                        result.push('R');
                        start += 1;
                    }
                }
            }

            result.push('!');
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1138_example_1() {
        let target = "leet".to_string();
        let result = "DDR!UURRR!!DDD!".to_string();

        assert_eq!(Solution::alphabet_board_path(target), result);
    }

    #[test]
    fn test_1138_example_2() {
        let target = "code".to_string();
        let result = "RR!DDRR!UUL!R!".to_string();

        assert_eq!(Solution::alphabet_board_path(target), result);
    }
}
