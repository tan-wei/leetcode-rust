/**
 * [0730] Count Different Palindromic Subsequences
 *
 * Given a string s, return the number of different non-empty palindromic subsequences in s. Since the answer may be very large, return it modulo 10^9 + 7.
 * A subsequence of a string is obtained by deleting zero or more characters from the string.
 * A sequence is palindromic if it is equal to the sequence reversed.
 * Two sequences a1, a2, ... and b1, b2, ... are different if there is some i for which ai != bi.
 *  
 * Example 1:
 *
 * Input: s = "bccb"
 * Output: 6
 * Explanation: The 6 different non-empty palindromic subsequences are 'b', 'c', 'bb', 'cc', 'bcb', 'bccb'.
 * Note that 'bcb' is counted only once, even though it occurs twice.
 *
 * Example 2:
 *
 * Input: s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
 * Output: 104860361
 * Explanation: There are 3104860382 different non-empty palindromic subsequences, which is 104860361 modulo 10^9 + 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s[i] is either 'a', 'b', 'c', or 'd'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-different-palindromic-subsequences/
// discuss: https://leetcode.com/problems/count-different-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let mut dp = vec![vec![0i64; n]; n];

        for len in 1..=n {
            for i in 0..n + 1 - len {
                let j = i + len - 1;
                if i == j {
                    dp[i][j] = 1;
                } else if i == j - 1 {
                    dp[i][j] = 2;
                } else {
                    if s[i] != s[j] {
                        dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                    } else {
                        let c = s[i];
                        let mut left = i + 1;
                        let mut right = j - 1;

                        while left < j && s[left] != c {
                            left += 1;
                        }

                        while right > i && s[right] != c {
                            right -= 1;
                        }

                        if left == j {
                            dp[i][j] = dp[i + 1][j - 1] * 2 + 2;
                        } else if left == right {
                            dp[i][j] = dp[i + 1][j - 1] * 2 + 1;
                        } else {
                            dp[i][j] = dp[i + 1][j - 1] * 2 - dp[left + 1][right - 1];
                        }
                    }
                }

                dp[i][j] = if dp[i][j] < 0 {
                    dp[i][j] + MOD
                } else {
                    dp[i][j] % MOD
                };
            }
        }

        dp[0][n - 1] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0730_example_1() {
        let s = "bccb".to_string();
        let result = 6;

        assert_eq!(Solution::count_palindromic_subsequences(s), result);
    }

    #[test]
    fn test_0730_example_2() {
        let s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string();
        let result = 104860361;

        assert_eq!(Solution::count_palindromic_subsequences(s), result);
    }
}
