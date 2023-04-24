/**
 * [1044] Longest Duplicate Substring
 *
 * Given a string s, consider all duplicated substrings: (contiguous) substrings of s that occur 2 or more times. The occurrences may overlap.
 * Return any duplicated substring that has the longest possible length. If s does not have a duplicated substring, the answer is "".
 *  
 * Example 1:
 * Input: s = "banana"
 * Output: "ana"
 * Example 2:
 * Input: s = "abcd"
 * Output: ""
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 3 * 10^4
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-duplicate-substring/
// discuss: https://leetcode.com/problems/longest-duplicate-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut max_len = 0;
        let mut lo = 0;

        let s_str = s.as_bytes();
        if s_str.iter().all(|&value| value == s_str[0]) {
            return s[1..s.len()].to_string();
        }

        let mut start = 0;
        for end in 1..s.len() {
            if max_len >= (end - start) {
                break;
            }

            let tmp = &s[start..end];

            if s[(start + 1)..s.len()].contains(tmp) && tmp.len() > max_len {
                lo = start;
                max_len = tmp.len();
            } else {
                start += 1;
            }
        }

        s[lo..(lo + max_len)].to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1044_example_1() {
        let s = "banana".to_string();
        let result = "ana".to_string();

        assert_eq!(Solution::longest_dup_substring(s), result);
    }

    #[test]
    fn test_1044_example_2() {
        let s = "abcd".to_string();
        let result = "".to_string();

        assert_eq!(Solution::longest_dup_substring(s), result);
    }
}
