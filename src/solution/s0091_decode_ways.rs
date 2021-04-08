/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z can be encoded into numbers using the following mapping:
 *
 * 'A' -> "1"
 * 'B' -> "2"
 * ...
 * 'Z' -> "26"
 *
 * To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
 *
 * 	"AAJF" with the grouping (1 1 10 6)
 * 	"KJF" with the grouping (11 10 6)
 *
 * Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
 * Given a string s containing only digits, return the number of ways to decode it.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: s = "12"
 * Output: 2
 * Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
 *
 * Example 2:
 *
 * Input: s = "226"
 * Output: 3
 * Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 *
 * Example 3:
 *
 * Input: s = "0"
 * Output: 0
 * Explanation: There is no character that is mapped to a number starting with 0.
 * The only valid mappings with 0 are 'J' -> "10" and 'T' -> "20", neither of which start with 0.
 * Hence, there are no valid ways to decode this since all digits need to be mapped.
 *
 * Example 4:
 *
 * Input: s = "06"
 * Output: 0
 * Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s contains only digits and may contain leading zero(s).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways/
// discuss: https://leetcode.com/problems/decode-ways/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![0i32; n + 1];
        let raw = s.as_bytes();

        dp[0] = 1;
        dp[1] = if raw[0] == b'0' {
            return 0;
        } else {
            1
        };

        for l in 2..=n {
            let idx = l - 1;
            dp[l] = if raw[idx] != b'0' { dp[l - 1] } else { 0 }
                + ({
                    let sum = raw[idx] - b'0' + (raw[idx - 1] - b'0') * 10;
                    if sum <= 26 && raw[idx - 1] != b'0' {
                        dp[l - 2]
                    } else {
                        0
                    }
                });
        }

        dp[n]
    }

    // Credit: https://leetcode.com/problems/decode-ways/discuss/987029/Rust-O(1)-space-oneliner-explained
    pub fn num_decodings_v2(s: String) -> i32 {
        s.chars()
            .rev()
            .fold((' ', 0, 1), |(p, np, n), c| {
                (
                    c,
                    n,
                    (if ('1'..='9').contains(&c) {
                        n + format!("{}{}", c, p)
                            .parse::<i32>()
                            .ok()
                            .filter(|&i| i <= 26)
                            .map(|_| np)
                            .unwrap_or(0)
                    } else {
                        0
                    }),
                )
            })
            .2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0091_example_1() {
        let s = "12".to_string();
        let result = 2;

        assert_eq!(Solution::num_decodings(s), result);

        let s = "12".to_string();
        let result = 2;

        assert_eq!(Solution::num_decodings_v2(s), result);
    }

    #[test]
    fn test_0091_example_2() {
        let s = "12".to_string();
        let result = 2;

        assert_eq!(Solution::num_decodings(s), result);

        let s = "12".to_string();
        let result = 2;

        assert_eq!(Solution::num_decodings_v2(s), result);
    }

    #[test]
    fn test_0091_example_3() {
        let s = "0".to_string();
        let result = 0;

        assert_eq!(Solution::num_decodings(s), result);

        let s = "0".to_string();
        let result = 0;

        assert_eq!(Solution::num_decodings_v2(s), result);
    }

    #[test]
    fn test_0091_example_4() {
        let s = "06".to_string();
        let result = 0;

        assert_eq!(Solution::num_decodings(s), result);

        let s = "06".to_string();
        let result = 0;

        assert_eq!(Solution::num_decodings_v2(s), result);
    }
}
