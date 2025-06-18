/**
 * [2047] Number of Valid Words in a Sentence
 *
 * A sentence consists of lowercase letters ('a' to 'z'), digits ('0' to '9'), hyphens ('-'), punctuation marks ('!', '.', and ','), and spaces (' ') only. Each sentence can be broken down into one or more tokens separated by one or more spaces ' '.
 * A token is a valid word if all three of the following are true:
 *
 * 	It only contains lowercase letters, hyphens, and/or punctuation (no digits).
 * 	There is at most one hyphen '-'. If present, it must be surrounded by lowercase characters ("a-b" is valid, but "-ab" and "ab-" are not valid).
 * 	There is at most one punctuation mark. If present, it must be at the end of the token ("ab,", "cd!", and "." are valid, but "a!b" and "c.," are not valid).
 *
 * Examples of valid words include "a-b.", "afad", "ba-c", "a!", and "!".
 * Given a string sentence, return the number of valid words in sentence.
 *  
 * Example 1:
 *
 * Input: sentence = "<u>cat</u> <u>and</u>  <u>dog</u>"
 * Output: 3
 * Explanation: The valid words in the sentence are "cat", "and", and "dog".
 *
 * Example 2:
 *
 * Input: sentence = "!this  1-s b8d!"
 * Output: 0
 * Explanation: There are no valid words in the sentence.
 * "!this" is invalid because it starts with a punctuation mark.
 * "1-s" and "b8d" are invalid because they contain digits.
 *
 * Example 3:
 *
 * Input: sentence = "<u>alice</u> <u>and</u>  <u>bob</u> <u>are</u> <u>playing</u> stone-game10"
 * Output: 5
 * Explanation: The valid words in the sentence are "alice", "and", "bob", "are", and "playing".
 * "stone-game10" is invalid because it contains digits.
 *
 *  
 * Constraints:
 *
 * 	1 <= sentence.length <= 1000
 * 	sentence only contains lowercase English letters, digits, ' ', '-', '!', '.', and ','.
 * 	There will be at least 1 token.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-valid-words-in-a-sentence/
// discuss: https://leetcode.com/problems/number-of-valid-words-in-a-sentence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-valid-words-in-a-sentence/solutions/1538215/c-rust-solution/
    pub fn count_valid_words(sentence: String) -> i32 {
        let rule1 = |w: &str| w.bytes().all(|c| !c.is_ascii_digit());
        let rule2 = |w: &str| {
            if let Some(pos) = w.find('-') {
                0 < pos
                    && pos < w.len() - 1
                    && w.as_bytes()[pos + 1].is_ascii_lowercase()
                    && w[pos + 1..].find('-').is_none()
            } else {
                true
            }
        };
        let rule3 = |w: &str| {
            w[..w.len() - 1]
                .bytes()
                .all(|c| c != b'!' && c != b'.' && c != b',')
        };

        sentence
            .split(' ')
            .filter(|w| !w.is_empty() && rule1(w) && rule2(w) && rule3(w))
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2047_example_1() {
        let sentence = "cat and  dog".to_string();

        let result = 3;

        assert_eq!(Solution::count_valid_words(sentence), result);
    }

    #[test]
    fn test_2047_example_2() {
        let sentence = "!this  1-s b8d!".to_string();

        let result = 0;

        assert_eq!(Solution::count_valid_words(sentence), result);
    }

    #[test]
    fn test_2047_example_3() {
        let sentence = "alice and  bob are playing stone-game10".to_string();

        let result = 5;

        assert_eq!(Solution::count_valid_words(sentence), result);
    }
}
