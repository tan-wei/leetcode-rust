/**
 * [0488] Zuma Game
 *
 * You are playing a variation of the game Zuma.
 * In this variation of Zuma, there is a single row of colored balls on a board, where each ball can be colored red 'R', yellow 'Y', blue 'B', green 'G', or white 'W'. You also have several colored balls in your hand.
 * Your goal is to clear all of the balls from the board. On each turn:
 *
 * 	Pick any ball from your hand and insert it in between two balls in the row or on either end of the row.
 * 	If there is a group of three or more consecutive balls of the same color, remove the group of balls from the board.
 *
 * 		If this removal causes more groups of three or more of the same color to form, then continue removing each group until there are none left.
 *
 *
 * 	If there are no more balls on the board, then you win the game.
 * 	Repeat this process until you either win or do not have any more balls in your hand.
 *
 * Given a string board, representing the row of balls on the board, and a string hand, representing the balls in your hand, return the minimum number of balls you have to insert to clear all the balls from the board. If you cannot clear all the balls from the board using the balls in your hand, return -1.
 *  
 * Example 1:
 *
 * Input: board = "WRRBBW", hand = "RB"
 * Output: -1
 * Explanation: It is impossible to clear all the balls. The best you can do is:
 * - Insert 'R' so the board becomes WRR<u>R</u>BBW. W<u>RRR</u>BBW -> WBBW.
 * - Insert 'B' so the board becomes WBB<u>B</u>W. W<u>BBB</u>W -> WW.
 * There are still balls remaining on the board, and you are out of balls to insert.
 * Example 2:
 *
 * Input: board = "WWRRBBWW", hand = "WRBRW"
 * Output: 2
 * Explanation: To make the board empty:
 * - Insert 'R' so the board becomes WWRR<u>R</u>BBWW. WW<u>RRR</u>BBWW -> WWBBWW.
 * - Insert 'B' so the board becomes WWBB<u>B</u>WW. WW<u>BBB</u>WW -> <u>WWWW</u> -> empty.
 * 2 balls from your hand were needed to clear the board.
 *
 * Example 3:
 *
 * Input: board = "G", hand = "GGGGG"
 * Output: 2
 * Explanation: To make the board empty:
 * - Insert 'G' so the board becomes G<u>G</u>.
 * - Insert 'G' so the board becomes GG<u>G</u>. <u>GGG</u> -> empty.
 * 2 balls from your hand were needed to clear the board.
 *
 *  
 * Constraints:
 *
 * 	1 <= board.length <= 16
 * 	1 <= hand.length <= 5
 * 	board and hand consist of the characters 'R', 'Y', 'B', 'G', and 'W'.
 * 	The initial row of balls on the board will not have any groups of three or more consecutive balls of the same color.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zuma-game/
// discuss: https://leetcode.com/problems/zuma-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn find_min_step(board: String, hand: String) -> i32 {
        let n = hand.len();
        let board: Vec<char> = board.chars().collect();
        let hand: Vec<char> = hand.chars().collect();
        let mut res = std::i32::MAX;
        Self::dfs(0, 0, board, &mut res, &hand, n);
        if res == std::i32::MAX {
            -1
        } else {
            res
        }
    }

    fn dfs(start: usize, state: u32, board: Vec<char>, res: &mut i32, hand: &[char], n: usize) {
        if start == n {
            return;
        }
        for i in 0..board.len() {
            if i == 0 || board[i] != board[i - 1] {
                if let Some(next_state) = Self::find_next_state(board[i], state, hand, n) {
                    let mut new_board: Vec<char> = board.to_vec();
                    new_board.insert(i, board[i]);
                    while let Some(range) = Self::dropable(&new_board) {
                        new_board.drain(range);
                        Self::dfs(start + 1, next_state, new_board.to_vec(), res, hand, n);
                    }
                    if new_board.is_empty() {
                        *res = (*res).min((start + 1) as i32);
                    } else {
                        Self::dfs(start + 1, next_state, new_board, res, hand, n);
                    }
                }
            }
        }
    }

    fn find_next_state(c: char, state: u32, hand: &[char], n: usize) -> Option<u32> {
        for i in 0..n {
            if hand[i] == c && state & 1 << i == 0 {
                return Some(state | 1 << i);
            }
        }
        None
    }

    fn dropable(board: &[char]) -> Option<std::ops::Range<usize>> {
        let n = board.len();
        let mut l = 0;
        let mut r = 0;
        while r < n {
            if board[l] == board[r] {
                r += 1;
            } else {
                if r - l >= 3 {
                    return Some(l..r);
                } else {
                    l = r;
                }
            }
        }
        if r - l >= 3 {
            return Some(l..r);
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0488_example_1() {
        let board = "WRRBBW".to_string();
        let hand = "RB".to_string();
        let result = -1;

        assert_eq!(Solution::find_min_step(board, hand), result);
    }

    #[test]
    fn test_0488_example_2() {
        let board = "WWRRBBWW".to_string();
        let hand = "WRBRW".to_string();
        let result = 2;

        assert_eq!(Solution::find_min_step(board, hand), result);
    }

    #[test]
    fn test_0488_example_3() {
        let board = "G".to_string();
        let hand = "GG".to_string();
        let result = 2;

        assert_eq!(Solution::find_min_step(board, hand), result);
    }

    #[ignore]
    #[test]
    fn test_0488_addtional_1() {
        let board = "WWBBWBBWW".to_string();
        let hand = "BB".to_string();
        let result = -1;

        assert_eq!(Solution::find_min_step(board, hand), result);
    }
}
