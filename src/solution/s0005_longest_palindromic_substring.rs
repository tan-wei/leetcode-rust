/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 *  
 * Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 * Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 * Example 3:
 *
 * Input: s = "a"
 * Output: "a"
 *
 * Example 4:
 *
 * Input: s = "ac"
 * Output: "a"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters (lower-case and/or upper-case),
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 {
            return s;
        }

        let mut begin = 0;
        let mut end = 0;

        for s in 0..chars.len() {
            for e in s + 1..chars.len() {
                if Solution::is_palindrome(&chars, s, e) && e - s > end - begin {
                    end = e;
                    begin = s;
                }
            }
        }
        s[begin..end + 1].into()
    }

    fn is_palindrome(s: &Vec<char>, begin: usize, end: usize) -> bool {
        let mut end = end;
        let mut begin = begin;
        while begin < end {
            if s[begin] != s[end] {
                return false;
            }
            begin += 1;
            end -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005_example_1() {
        let s = "babad".to_string();

        assert_eq!(Solution::longest_palindrome(s), "bab".to_string());
    }

    #[test]
    fn test_0005_example_2() {
        let s = "cbbd".to_string();

        assert_eq!(Solution::longest_palindrome(s), "bb".to_string());
    }

    #[test]
    fn test_0005_example_3() {
        let s = "a".to_string();

        assert_eq!(Solution::longest_palindrome(s), "a".to_string());
    }

    #[test]
    fn test_0005_example_4() {
        let s = "ac".to_string();

        assert_eq!(Solution::longest_palindrome(s), "a".to_string());
    }

    #[test]
    fn test_0005_empty_string() {
        let s = String::new();

        assert_eq!(Solution::longest_palindrome(s), String::new());
    }
}
