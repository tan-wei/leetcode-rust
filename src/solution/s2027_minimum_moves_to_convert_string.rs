/**
 * [2027] Minimum Moves to Convert String
 *
 * You are given a string s consisting of n characters which are either 'X' or 'O'.
 * A move is defined as selecting three consecutive characters of s and converting them to 'O'. Note that if a move is applied to the character 'O', it will stay the same.
 * Return the minimum number of moves required so that all the characters of s are converted to 'O'.
 *  
 * Example 1:
 *
 * Input: s = "XXX"
 * Output: 1
 * Explanation: <u>XXX</u> -> OOO
 * We select all the 3 characters and convert them in one move.
 *
 * Example 2:
 *
 * Input: s = "XXOX"
 * Output: 2
 * Explanation: <u>XXO</u>X -> O<u>OOX</u> -> OOOO
 * We select the first 3 characters in the first move, and convert them to 'O'.
 * Then we select the last 3 characters and convert them so that the final string contains all 'O's.
 * Example 3:
 *
 * Input: s = "OOOO"
 * Output: 0
 * Explanation: There are no 'X's in s to convert.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 1000
 * 	s[i] is either 'X' or 'O'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-convert-string/
// discuss: https://leetcode.com/problems/minimum-moves-to-convert-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut result = 0;
        let mut i = 0;
        let s = s.chars().collect::<Vec<_>>();

        while i < s.len() {
            if s[i] == 'X' {
                i += 3;
                result += 1
            } else {
                i += 1
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
    fn test_2027_example_1() {
        let s = "XXX".to_string();

        let result = 1;

        assert_eq!(Solution::minimum_moves(s), result);
    }

    #[test]
    fn test_2027_example_2() {
        let s = "XXOX".to_string();

        let result = 2;

        assert_eq!(Solution::minimum_moves(s), result);
    }

    #[test]
    fn test_2027_example_3() {
        let s = "OOOO".to_string();

        let result = 0;

        assert_eq!(Solution::minimum_moves(s), result);
    }
}
