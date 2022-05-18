/**
 * [0680] Valid Palindrome II
 *
 * Given a string s, return true if the s can be palindrome after deleting at most one character from it.
 *  
 * Example 1:
 *
 * Input: s = "aba"
 * Output: true
 *
 * Example 2:
 *
 * Input: s = "abca"
 * Output: true
 * Explanation: You could delete the character 'c'.
 *
 * Example 3:
 *
 * Input: s = "abc"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome-ii/
// discuss: https://leetcode.com/problems/valid-palindrome-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if chars[left] != chars[right] {
                let (l, r) = (&chars[left..right], &chars[left + 1..right + 1]);
                return (0..l.len()).all(|i| l[i] == l[l.len() - 1 - i])
                    || (0..r.len()).all(|i| r[i] == r[r.len() - 1 - i]);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0680_example_1() {
        let s = "aba".to_string();
        let result = true;

        assert_eq!(Solution::valid_palindrome(s), result);
    }

    #[test]
    fn test_0680_example_2() {
        let s = "abca".to_string();
        let result = true;

        assert_eq!(Solution::valid_palindrome(s), result);
    }

    #[test]
    fn test_0680_example_3() {
        let s = "abc".to_string();
        let result = false;

        assert_eq!(Solution::valid_palindrome(s), result);
    }
}
