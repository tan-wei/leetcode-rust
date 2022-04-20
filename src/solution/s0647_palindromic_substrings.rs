/**
 * [0647] Palindromic Substrings
 *
 * Given a string s, return the number of palindromic substrings in it.
 * A string is a palindrome when it reads the same backward as forward.
 * A substring is a contiguous sequence of characters within the string.
 *  
 * Example 1:
 *
 * Input: s = "abc"
 * Output: 3
 * Explanation: Three palindromic strings: "a", "b", "c".
 *
 * Example 2:
 *
 * Input: s = "aaa"
 * Output: 6
 * Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindromic-substrings/
// discuss: https://leetcode.com/problems/palindromic-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();

        (0..s.len())
            .map(|i| Self::count_parindrome(&s, i, i) + Self::count_parindrome(&s, i, i + 1))
            .sum()
    }

    fn count_parindrome(s: &[u8], i: usize, j: usize) -> i32 {
        if j == s.len() || s[i] != s[j] {
            return 0;
        }

        let (mut i, mut j) = (i, j);
        let mut result = 1;

        while i > 0 && j < s.len() - 1 && s[i - 1] == s[j + 1] {
            result += 1;
            i -= 1;
            j += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0647_example_1() {
        let s = "abc".to_string();
        let result = 3;

        assert_eq!(Solution::count_substrings(s), result);
    }

    #[test]
    fn test_0647_example_2() {
        let s = "aaa".to_string();
        let result = 6;

        assert_eq!(Solution::count_substrings(s), result);
    }
}
