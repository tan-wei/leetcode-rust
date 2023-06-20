/**
 * [1163] Last Substring in Lexicographical Order
 *
 * Given a string s, return the last substring of s in lexicographical order.
 *  
 * Example 1:
 *
 * Input: s = "abab"
 * Output: "bab"
 * Explanation: The substrings are ["a", "ab", "aba", "abab", "b", "ba", "bab"]. The lexicographically maximum substring is "bab".
 *
 * Example 2:
 *
 * Input: s = "leetcode"
 * Output: "tcode"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 4 * 10^5
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-substring-in-lexicographical-order/
// discuss: https://leetcode.com/problems/last-substring-in-lexicographical-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/last-substring-in-lexicographical-order/solutions/3058847/just-a-runnable-solution/
    pub fn last_substring(s: String) -> String {
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        let s = s.as_bytes();
        let n = s.len();
        while j + k < n {
            match s[i + k].cmp(&s[j + k]) {
                std::cmp::Ordering::Equal => k += 1,
                std::cmp::Ordering::Less => {
                    i = (i + k + 1).max(j);
                    j = i + 1;
                    k = 0;
                }
                std::cmp::Ordering::Greater => {
                    j += k + 1;
                    k = 0;
                }
            }
        }
        unsafe { String::from_utf8_unchecked(s[i..].to_vec()) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1163_example_1() {
        let s = "abab".to_string();
        let result = "bab".to_string();

        assert_eq!(Solution::last_substring(s), result);
    }

    #[test]
    fn test_1163_example_2() {
        let s = "leetcode".to_string();
        let result = "tcode".to_string();

        assert_eq!(Solution::last_substring(s), result);
    }
}
