/**
 * [1668] Maximum Repeating Substring
 *
 * For a string sequence, a string word is k-repeating if word concatenated k times is a substring of sequence. The word's maximum k-repeating value is the highest value k where word is k-repeating in sequence. If word is not a substring of sequence, word's maximum k-repeating value is 0.
 * Given strings sequence and word, return the maximum k-repeating value of word in sequence.
 *  
 * Example 1:
 *
 * Input: sequence = "ababc", word = "ab"
 * Output: 2
 * Explanation: "abab" is a substring in "<u>abab</u>c".
 *
 * Example 2:
 *
 * Input: sequence = "ababc", word = "ba"
 * Output: 1
 * Explanation: "ba" is a substring in "a<u>ba</u>bc". "baba" is not a substring in "ababc".
 *
 * Example 3:
 *
 * Input: sequence = "ababc", word = "ac"
 * Output: 0
 * Explanation: "ac" is not a substring in "ababc".
 *
 *  
 * Constraints:
 *
 * 	1 <= sequence.length <= 100
 * 	1 <= word.length <= 100
 * 	sequence and word contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-repeating-substring/
// discuss: https://leetcode.com/problems/maximum-repeating-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        sequence
            .chars()
            .skip(word.len() - 1)
            .enumerate()
            .map(|(i, _)| Self::dfs_helper(&sequence[i..], &word))
            .max()
            .unwrap_or_default()
    }

    fn dfs_helper(sequence: &str, word: &str) -> i32 {
        sequence
            .strip_prefix(word)
            .map(|rest| Self::dfs_helper(rest, word) + 1)
            .unwrap_or_default()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1668_example_1() {
        let sequence = "ababc".to_string();
        let word = "ab".to_string();

        let result = 2;

        assert_eq!(Solution::max_repeating(sequence, word), result);
    }

    #[test]
    fn test_1668_example_2() {
        let sequence = "ababc".to_string();
        let word = "ba".to_string();

        let result = 1;

        assert_eq!(Solution::max_repeating(sequence, word), result);
    }

    #[test]
    fn test_1668_example_3() {
        let sequence = "ababc".to_string();
        let word = "ac".to_string();

        let result = 0;

        assert_eq!(Solution::max_repeating(sequence, word), result);
    }
}
