/**
 * [0409] Longest Palindrome
 *
 * Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
 * Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
 *  
 * Example 1:
 *
 * Input: s = "abccccdd"
 * Output: 7
 * Explanation:
 * One longest palindrome that can be built is "dccaccd", whose length is 7.
 *
 * Example 2:
 *
 * Input: s = "a"
 * Output: 1
 *
 * Example 3:
 *
 * Input: s = "bb"
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists of lowercase and/or uppercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome/
// discuss: https://leetcode.com/problems/longest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = std::collections::HashMap::new();
        let mut result = 0;
        for ch in s.chars() {
            let i = count.entry(ch).or_insert(0);
            *i += 1;
        }
        for v in count.values() {
            if v % 2 == 0 || result % 2 == 0 {
                result += v;
            } else {
                result += v - 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0409_example_1() {
        let s = "abccccdd".to_string();
        let result = 7;

        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn test_0409_example_2() {
        let s = "a".to_string();
        let result = 1;

        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn test_0409_example_3() {
        let s = "bb".to_string();
        let result = 2;

        assert_eq!(Solution::longest_palindrome(s), result);
    }
}
