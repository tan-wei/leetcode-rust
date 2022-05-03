/**
 * [0664] Strange Printer
 *
 * There is a strange printer with the following two special properties:
 *
 * 	The printer can only print a sequence of the same character each time.
 * 	At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.
 *
 * Given a string s, return the minimum number of turns the printer needed to print it.
 *  
 * Example 1:
 *
 * Input: s = "aaabbb"
 * Output: 2
 * Explanation: Print "aaa" first and then print "bbb".
 *
 * Example 2:
 *
 * Input: s = "aba"
 * Output: 2
 * Explanation: Print "aaa" first and then print "b" from the second place of the string, which will cover the existing character 'a'.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strange-printer/
// discuss: https://leetcode.com/problems/strange-printer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        match n {
            0 => 0,
            n @ _ => {
                let mut dp = vec![vec![0; n + 1]; n + 1];
                for len in 1..=n {
                    for i in 0..(n - len + 1) {
                        let j = i + len - 1;
                        dp[i][j] = 1 + dp[i + 1][j];
                        for k in (i + 1)..=j {
                            if s[k] == s[i] {
                                dp[i][j] = std::cmp::min(dp[i][j], dp[i][k - 1] + dp[k + 1][j]);
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
    fn test_0664_example_1() {
        let s = "aaabbb".to_string();
        let result = 2;

        assert_eq!(Solution::strange_printer(s), result);
    }

    #[test]
    fn test_0664_example_2() {
        let s = "aba".to_string();
        let result = 2;

        assert_eq!(Solution::strange_printer(s), result);
    }
}
