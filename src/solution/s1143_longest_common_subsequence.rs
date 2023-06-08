/**
 * [1143] Longest Common Subsequence
 *
 * Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 *
 * 	For example, "ace" is a subsequence of "abcde".
 *
 * A common subsequence of two strings is a subsequence that is common to both strings.
 *  
 * Example 1:
 *
 * Input: text1 = "abcde", text2 = "ace"
 * Output: 3  
 * Explanation: The longest common subsequence is "ace" and its length is 3.
 *
 * Example 2:
 *
 * Input: text1 = "abc", text2 = "abc"
 * Output: 3
 * Explanation: The longest common subsequence is "abc" and its length is 3.
 *
 * Example 3:
 *
 * Input: text1 = "abc", text2 = "def"
 * Output: 0
 * Explanation: There is no such common subsequence, so the result is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= text1.length, text2.length <= 1000
 * 	text1 and text2 consist of only lowercase English characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-subsequence/
// discuss: https://leetcode.com/problems/longest-common-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s1 = text1.chars().collect::<Vec<char>>();
        let s2 = text2.chars().collect::<Vec<char>>();

        let n = s1.len();
        let m = s2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j + 1], dp[i][j] + 1);
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]).max(dp[i][j + 1]);
                }
            }
        }
        dp[n][m]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1143_example_1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let result = 3;

        assert_eq!(Solution::longest_common_subsequence(text1, text2), result);
    }

    #[test]
    fn test_1143_example_2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        let result = 3;

        assert_eq!(Solution::longest_common_subsequence(text1, text2), result);
    }

    #[test]
    fn test_1143_example_3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        let result = 0;

        assert_eq!(Solution::longest_common_subsequence(text1, text2), result);
    }
}
