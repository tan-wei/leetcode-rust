/**
 * [44] Wildcard Matching
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:
 *
 * 	'?' Matches any single character.
 * 	'*' Matches any sequence of characters (including the empty sequence).
 *
 * The matching should cover the entire input string (not partial).
 *  
 * Example 1:
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * Example 2:
 *
 * Input: s = "aa", p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 *
 * Example 3:
 *
 * Input: s = "cb", p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 *
 * Example 4:
 *
 * Input: s = "adceb", p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 *
 * Example 5:
 *
 * Input: s = "acdcb", p = "a*c?b"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length, p.length <= 2000
 * 	s contains only lowercase English letters.
 * 	p contains only lowercase English letters, '?' or '*'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wildcard-matching/
// discuss: https://leetcode.com/problems/wildcard-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/wildcard-matching/discuss/442999/rust-solution-DP-O(n)-space
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        if s.is_empty() {
            return p.is_empty() || p.iter().all(|x| *x == b'*');
        } else if p.is_empty() {
            return false;
        }

        let mut dp = vec![false; p.len() + 1];
        dp[0] = true;
        for j in 1..dp.len() {
            dp[j] = if p[j - 1] == b'*' { dp[j - 1] } else { break };
        }
        for i in 1..=s.len() {
            let mut dp_i_1_j_1 = dp[0];
            for j in 1..dp.len() {
                let saved = dp[j];
                dp[j] = if s[i - 1] == p[j - 1] || p[j - 1] == b'?' {
                    dp_i_1_j_1
                } else if p[j - 1] == b'*' {
                    dp[j] || dp[j - 1]
                } else {
                    false
                };
                dp_i_1_j_1 = saved;
            }
            if i == 1 {
                dp[0] = false;
            }
        }
        *dp.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0044_example_1() {
        let s = "aa".to_string();
        let p = "a".to_string();

        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn test_0044_example_2() {
        let s = "aa".to_string();
        let p = "*".to_string();

        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn test_0044_example_3() {
        let s = "cb".to_string();
        let p = "?a".to_string();

        assert_eq!(Solution::is_match(s, p), false);
    }

    #[test]
    fn test_0044_example_4() {
        let s = "adceb".to_string();
        let p = "*a*b".to_string();

        assert_eq!(Solution::is_match(s, p), true);
    }

    #[test]
    fn test_0044_example_5() {
        let s = "acdcb".to_string();
        let p = "a*c?b".to_string();

        assert_eq!(Solution::is_match(s, p), false);
    }
}
