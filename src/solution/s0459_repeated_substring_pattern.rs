/**
 * [0459] Repeated Substring Pattern
 *
 * Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
 *  
 * Example 1:
 *
 * Input: s = "abab"
 * Output: true
 * Explanation: It is the substring "ab" twice.
 *
 * Example 2:
 *
 * Input: s = "aba"
 * Output: false
 *
 * Example 3:
 *
 * Input: s = "abcabcabcabc"
 * Output: true
 * Explanation: It is the substring "abc" four times or the substring "abcabc" twice.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-substring-pattern/
// discuss: https://leetcode.com/problems/repeated-substring-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        (s.clone() + &s)[1..s.len() * 2 - 1].contains(&s)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0459_example_1() {
        let s = "abab".to_string();
        let result = true;

        assert_eq!(Solution::repeated_substring_pattern(s), result);
    }

    #[test]
    fn test_0459_example_2() {
        let s = "aba".to_string();
        let result = false;

        assert_eq!(Solution::repeated_substring_pattern(s), result);
    }

    #[test]
    fn test_0459_example_3() {
        let s = "abcabcabcabc".to_string();
        let result = true;

        assert_eq!(Solution::repeated_substring_pattern(s), result);
    }
}
