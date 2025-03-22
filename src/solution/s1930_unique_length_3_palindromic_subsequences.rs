/**
 * [1930] Unique Length-3 Palindromic Subsequences
 *
 * Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
 * Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
 * A palindrome is a string that reads the same forwards and backwards.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 *
 * 	For example, "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>".
 *
 *  
 * Example 1:
 *
 * Input: s = "aabca"
 * Output: 3
 * Explanation: The 3 palindromic subsequences of length 3 are:
 * - "aba" (subsequence of "<u>a</u>a<u>b</u>c<u>a</u>")
 * - "aaa" (subsequence of "<u>aa</u>bc<u>a</u>")
 * - "aca" (subsequence of "<u>a</u>ab<u>ca</u>")
 *
 * Example 2:
 *
 * Input: s = "adc"
 * Output: 0
 * Explanation: There are no palindromic subsequences of length 3 in "adc".
 *
 * Example 3:
 *
 * Input: s = "bbcbaba"
 * Output: 4
 * Explanation: The 4 palindromic subsequences of length 3 are:
 * - "bbb" (subsequence of "<u>bb</u>c<u>b</u>aba")
 * - "bcb" (subsequence of "<u>b</u>b<u>cb</u>aba")
 * - "bab" (subsequence of "<u>b</u>bcb<u>ab</u>a")
 * - "aba" (subsequence of "bbcb<u>aba</u>")
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 10^5
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
// discuss: https://leetcode.com/problems/unique-length-3-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/unique-length-3-palindromic-subsequences/solutions/4287500/7-ms-beats-100-of-rust-solutions-in-both-time-and-memory/
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut char_positions = [(usize::MAX, 0_usize); 26];

        for (i, b) in s.bytes().enumerate() {
            let bu = usize::from(b - b'a');
            char_positions[bu].0 = char_positions[bu].0.min(i);
            char_positions[bu].1 = i;
        }

        let bs = s.as_bytes();
        char_positions
            .iter()
            .filter(|(f, l)| f < l)
            .map(|(f, l)| {
                bs[f + 1..*l]
                    .iter()
                    .fold(0_u32, |set, c| set | (1 << (c - b'a')))
                    .count_ones() as i32
            })
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1930_example_1() {
        let s = "aabca".to_string();

        let result = 3;

        assert_eq!(Solution::count_palindromic_subsequence(s), result);
    }

    #[test]
    fn test_1930_example_2() {
        let s = "adc".to_string();

        let result = 0;

        assert_eq!(Solution::count_palindromic_subsequence(s), result);
    }

    #[test]
    fn test_1930_example_3() {
        let s = "bbcbaba".to_string();

        let result = 4;

        assert_eq!(Solution::count_palindromic_subsequence(s), result);
    }
}
