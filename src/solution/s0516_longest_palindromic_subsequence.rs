/**
 * [0516] Longest Palindromic Subsequence
 *
 * Given a string s, find the longest palindromic subsequence's length in s.
 * A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: s = "bbbab"
 * Output: 4
 * Explanation: One possible longest palindromic subsequence is "bbbb".
 *
 * Example 2:
 *
 * Input: s = "cbbd"
 * Output: 2
 * Explanation: One possible longest palindromic subsequence is "bb".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-subsequence/
// discuss: https://leetcode.com/problems/longest-palindromic-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        match s.len() {
            0 => 0,
            n => {
                let s = s.chars().collect::<Vec<_>>();
                let mut dp = vec![vec![1; n]; n];

                for j in 1..n {
                    for i in (0..=(j - 1)).rev() {
                        if s[i] == s[j] {
                            if i + 1 <= j - 1 {
                                dp[i][j] = dp[i + 1][j - 1] + 2;
                            } else {
                                dp[i][j] = 2;
                            }
                        } else {
                            if i + 1 < n && j >= 1 {
                                dp[i][j] = dp[i][j - 1].max(dp[i + 1][j]);
                            } else if i + 1 < n {
                                dp[i][j] = dp[i + 1][j];
                            } else if j >= 1 {
                                dp[i][j] = dp[i][j - 1];
                            }
                        }
                    }
                }
                dp[0][n - 1]
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0516_example_1() {
        let s = "bbbab".to_string();
        let result = 4;

        assert_eq!(Solution::longest_palindrome_subseq(s), result);
    }

    #[test]
    fn test_0516_example_2() {
        let s = "cbbd".to_string();
        let result = 2;

        assert_eq!(Solution::longest_palindrome_subseq(s), result);
    }
}
