/**
 * [2486] Append Characters to String to Make Subsequence
 *
 * You are given two strings s and t consisting of only lowercase English letters.
 * Return the minimum number of characters that need to be appended to the end of s so that t becomes a subsequence of s.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 *  
 * Example 1:
 *
 * Input: s = "coaching", t = "coding"
 * Output: 4
 * Explanation: Append the characters "ding" to the end of s so that s = "coachingding".
 * Now, t is a subsequence of s ("<u>co</u>aching<u>ding</u>").
 * It can be shown that appending any 3 characters to the end of s will never make t a subsequence.
 *
 * Example 2:
 *
 * Input: s = "abcde", t = "a"
 * Output: 0
 * Explanation: t is already a subsequence of s ("<u>a</u>bcde").
 *
 * Example 3:
 *
 * Input: s = "z", t = "abcde"
 * Output: 5
 * Explanation: Append the characters "abcde" to the end of s so that s = "zabcde".
 * Now, t is a subsequence of s ("z<u>abcde</u>").
 * It can be shown that appending any 4 characters to the end of s will never make t a subsequence.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 10^5
 * 	s and t consist only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/
// discuss: https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2486_example_1() {
        let s = "coaching".to_string();
        let t = "coding".to_string();

        let result = 4;

        assert_eq!(Solution::append_characters(s, t), result);
    }

    #[test]
    #[ignore]
    fn test_2486_example_2() {
        let s = "abcde".to_string();
        let t = "a".to_string();

        let result = 0;

        assert_eq!(Solution::append_characters(s, t), result);
    }

    #[test]
    #[ignore]
    fn test_2486_example_3() {
        let s = "z".to_string();
        let t = "abcde".to_string();

        let result = 0;

        assert_eq!(Solution::append_characters(s, t), result);
    }
}
