/**
 * [125] Valid Palindrome
 *
 * Given a string s, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
 *  
 * Example 1:
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 * Example 2:
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2 * 10^5
 * 	s consists only of printable ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        s.chars()
            .zip(s.chars().rev())
            .fold(true, |acc, (a, b)| acc & (a == b))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0125_example_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let result = true;

        assert_eq!(Solution::is_palindrome(s), result);
    }

    #[test]
    fn test_0125_example_2() {
        let s = "race a car".to_string();
        let result = false;

        assert_eq!(Solution::is_palindrome(s), result);
    }
}
