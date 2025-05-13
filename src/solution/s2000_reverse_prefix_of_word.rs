/**
 * [2000] Reverse Prefix of Word
 *
 * Given a 0-indexed string word and a character ch, reverse the segment of word that starts at index 0 and ends at the index of the first occurrence of ch (inclusive). If the character ch does not exist in word, do nothing.
 *
 * 	For example, if word = "abcdefd" and ch = "d", then you should reverse the segment that starts at 0 and ends at 3 (inclusive). The resulting string will be "<u>dcba</u>efd".
 *
 * Return the resulting string.
 *  
 * Example 1:
 *
 * Input: word = "<u>abcd</u>efd", ch = "d"
 * Output: "<u>dcba</u>efd"
 * Explanation: The first occurrence of "d" is at index 3.
 * Reverse the part of word from 0 to 3 (inclusive), the resulting string is "dcbaefd".
 *
 * Example 2:
 *
 * Input: word = "<u>xyxz</u>xe", ch = "z"
 * Output: "<u>zxyx</u>xe"
 * Explanation: The first and only occurrence of "z" is at index 3.
 * Reverse the part of word from 0 to 3 (inclusive), the resulting string is "zxyxxe".
 *
 * Example 3:
 *
 * Input: word = "abcd", ch = "z"
 * Output: "abcd"
 * Explanation: "z" does not exist in word.
 * You should not do any reverse operation, the resulting string is "abcd".
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 250
 * 	word consists of lowercase English letters.
 * 	ch is a lowercase English letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-prefix-of-word/
// discuss: https://leetcode.com/problems/reverse-prefix-of-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.split_once(ch) {
            Some((pre, post)) => ch.to_string() + &pre.chars().rev().collect::<String>() + post,
            None => word,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2000_example_1() {
        let word = "abcdefd".to_string();
        let ch = 'd';

        let result = "dcbaefd".to_string();

        assert_eq!(Solution::reverse_prefix(word, ch), result);
    }

    #[test]
    fn test_2000_example_2() {
        let word = "xyxzxe".to_string();
        let ch = 'z';

        let result = "zxyxxe".to_string();

        assert_eq!(Solution::reverse_prefix(word, ch), result);
    }

    #[test]
    fn test_2000_example_3() {
        let word = "abcd".to_string();
        let ch = 'z';

        let result = "abcd".to_string();

        assert_eq!(Solution::reverse_prefix(word, ch), result);
    }
}
