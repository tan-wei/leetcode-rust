/**
 * [1745] Palindrome Partitioning IV
 *
 * Given a string s, return true if it is possible to split the string s into three non-empty palindromic substrings. Otherwise, return false.​​​​​
 * A string is said to be palindrome if it the same string when reversed.
 *  
 * Example 1:
 *
 * Input: s = "abcbdd"
 * Output: true
 * Explanation: "abcbdd" = "a" + "bcb" + "dd", and all three substrings are palindromes.
 *
 * Example 2:
 *
 * Input: s = "bcbddxy"
 * Output: false
 * Explanation: s cannot be split into 3 palindromes.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 2000
 * 	s​​​​​​ consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning-iv/
// discuss: https://leetcode.com/problems/palindrome-partitioning-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let s = s.chars().collect::<Vec<char>>();

        for i in 0..n {
            dp[i][i] = true;
        }

        for k in 1..n {
            for i in 0..n {
                if i + k >= n {
                    continue;
                }
                dp[i][i + k] = if s[i] == s[i + k] { true } else { false };
                if k > 2 {
                    dp[i][i + k] = dp[i][i + k] && dp[i + 1][i + k - 1];
                }
            }
        }

        for i in 0..n {
            for j in i + 2..n {
                if dp[0][i] && dp[i + 1][j - 1] && dp[j][n - 1] {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1745_example_1() {
        let s = "abcbdd".to_string();

        let result = true;

        assert_eq!(Solution::check_partitioning(s), result);
    }

    #[test]
    fn test_1745_example_2() {
        let s = "bcbddxy".to_string();

        let result = false;

        assert_eq!(Solution::check_partitioning(s), result);
    }
}
