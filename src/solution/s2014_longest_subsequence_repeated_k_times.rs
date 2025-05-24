/**
 * [2014] Longest Subsequence Repeated k Times
 *
 * You are given a string s of length n, and an integer k. You are tasked to find the longest subsequence repeated k times in string s.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 * A subsequence seq is repeated k times in the string s if seq * k is a subsequence of s, where seq * k represents a string constructed by concatenating seq k times.
 *
 * 	For example, "bba" is repeated 2 times in the string "bababcba", because the string "bbabba", constructed by concatenating "bba" 2 times, is a subsequence of the string "<u>b</u>a<u>bab</u>c<u>ba</u>".
 *
 * Return the longest subsequence repeated k times in string s. If multiple such subsequences are found, return the lexicographically largest one. If there is no such subsequence, return an empty string.
 *  
 * Example 1:
 * <img alt="example 1" src="https://assets.leetcode.com/uploads/2021/08/30/longest-subsequence-repeat-k-times.png" style="width: 457px; height: 99px;" />
 * Input: s = "letsleetcode", k = 2
 * Output: "let"
 * Explanation: There are two longest subsequences repeated 2 times: "let" and "ete".
 * "let" is the lexicographically largest one.
 *
 * Example 2:
 *
 * Input: s = "bb", k = 2
 * Output: "b"
 * Explanation: The longest subsequence repeated 2 times is "b".
 *
 * Example 3:
 *
 * Input: s = "ab", k = 2
 * Output: ""
 * Explanation: There is no subsequence repeated 2 times. Empty string is returned.
 *
 *  
 * Constraints:
 *
 * 	n == s.length
 * 	2 <= n, k <= 2000
 * 	2 <= n < k * 8
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subsequence-repeated-k-times/
// discuss: https://leetcode.com/problems/longest-subsequence-repeated-k-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2014_example_1() {
        let s = "letsleetcode".to_string();
        let k = 2;

        let result = "let".to_string();

        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), result);
    }

    #[test]
    #[ignore]
    fn test_2014_example_2() {
        let s = "bb".to_string();
        let k = 2;

        let result = "b".to_string();

        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), result);
    }

    #[test]
    #[ignore]
    fn test_2014_example_3() {
        let s = "ab".to_string();
        let k = 2;

        let result = "".to_string();

        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), result);
    }
}
