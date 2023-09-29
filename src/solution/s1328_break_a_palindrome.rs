/**
 * [1328] Break a Palindrome
 *
 * Given a palindromic string of lowercase English letters palindrome, replace exactly one character with any lowercase English letter so that the resulting string is not a palindrome and that it is the lexicographically smallest one possible.
 * Return the resulting string. If there is no way to replace a character to make it not a palindrome, return an empty string.
 * A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, a has a character strictly smaller than the corresponding character in b. For example, "abcc" is lexicographically smaller than "abcd" because the first position they differ is at the fourth character, and 'c' is smaller than 'd'.
 *
 * Example 1:
 *
 * Input: palindrome = "abccba"
 * Output: "aaccba"
 * Explanation: There are many ways to make "abccba" not a palindrome, such as "<u>z</u>bccba", "a<u>a</u>ccba", and "ab<u>a</u>cba".
 * Of all the ways, "aaccba" is the lexicographically smallest.
 *
 * Example 2:
 *
 * Input: palindrome = "a"
 * Output: ""
 * Explanation: There is no way to replace a single character to make "a" not a palindrome, so return an empty string.
 *
 *
 * Constraints:
 *
 * 	1 <= palindrome.length <= 1000
 * 	palindrome consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/break-a-palindrome/
// discuss: https://leetcode.com/problems/break-a-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut s = palindrome.chars().collect::<Vec<char>>();
        let n = s.len();

        if n == 1 {
            return format!("");
        }

        for i in 0..n / 2 {
            if s[i] != 'a' {
                s[i] = 'a';
                return s.into_iter().collect::<String>();
            }
        }

        s[n - 1] = 'b';
        s.into_iter().collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1328_example_1() {
        let palindrome = "abccba".to_string();
        let result = "aaccba".to_string();

        assert_eq!(Solution::break_palindrome(palindrome), result);
    }

    #[test]
    fn test_1328_example_2() {
        let palindrome = "a".to_string();
        let result = "".to_string();

        assert_eq!(Solution::break_palindrome(palindrome), result);
    }
}
