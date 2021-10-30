/**
 * [0392] Is Subsequence
 *
 * Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>" while "aec" is not).
 *  
 * Example 1:
 * Input: s = "abc", t = "ahbgdc"
 * Output: true
 * Example 2:
 * Input: s = "axc", t = "ahbgdc"
 * Output: false
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 100
 * 	0 <= t.length <= 10^4
 * 	s and t consist only of lowercase English letters.
 *
 *  
 * Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 10^9, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/is-subsequence/
// discuss: https://leetcode.com/problems/is-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
            match iter.find(|&p| p == c) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0392_example_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let result = true;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }

    #[test]
    fn test_0392_example_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let result = false;

        assert_eq!(Solution::is_subsequence(s, t), result);
    }
}
