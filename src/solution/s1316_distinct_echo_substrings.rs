/**
 * [1316] Distinct Echo Substrings
 *
 * Return the number of distinct non-empty substrings of text that can be written as the concatenation of some string with itself (i.e. it can be written as a + a where a is some string).
 *
 * Example 1:
 *
 * Input: text = "abcabcabc"
 * Output: 3
 * Explanation: The 3 substrings are "abcabc", "bcabca" and "cabcab".
 *
 * Example 2:
 *
 * Input: text = "leetcodeleetcode"
 * Output: 2
 * Explanation: The 2 substrings are "ee" and "leetcodeleetcode".
 *
 *
 * Constraints:
 *
 * 	1 <= text.length <= 2000
 * 	text has only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-echo-substrings/
// discuss: https://leetcode.com/problems/distinct-echo-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let mut s = std::collections::HashSet::new();
        let text = text.as_bytes();
        for i in 0..text.len() {
            let mut result = vec![];
            for j in (i..text.len()).step_by(2) {
                result.push(text[j]);
                if j + 1 < text.len() {
                    result.push(text[j + 1]);
                } else {
                    break;
                }
                if Self::check(&result) {
                    s.insert(result.clone());
                }
            }
        }
        s.len() as i32
    }

    fn check(s: &[u8]) -> bool {
        let mut j = s.len() / 2;
        for i in 0..(s.len() / 2) {
            if s[i] != s[j] {
                return false;
            }
            j += 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1316_example_1() {
        let text = "abcabcabc".to_string();
        let result = 3;

        assert_eq!(Solution::distinct_echo_substrings(text), result);
    }

    #[test]
    fn test_1316_example_2() {
        let text = "leetcodeleetcode".to_string();
        let result = 2;

        assert_eq!(Solution::distinct_echo_substrings(text), result);
    }
}
