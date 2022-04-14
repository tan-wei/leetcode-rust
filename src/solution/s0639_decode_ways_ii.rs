/**
 * [0639] Decode Ways II
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
 * In addition to the mapping above, an encoded message may contain the '*' character, which can represent any digit from '1' to '9' ('0' is excluded). For example, the encoded message "1*" may represent any of the encoded messages "11", "12", "13", "14", "15", "16", "17", "18", or "19". Decoding "1*" is equivalent to decoding any of the encoded messages it can represent.
 * Given a string s consisting of digits and '*' characters, return the number of ways to decode it.
 * Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: s = "*"
 * Output: 9
 * Explanation: The encoded message can represent any of the encoded messages "1", "2", "3", "4", "5", "6", "7", "8", or "9".
 * Each of these can be decoded to the strings "A", "B", "C", "D", "E", "F", "G", "H", and "I" respectively.
 * Hence, there are a total of 9 ways to decode "*".
 *
 * Example 2:
 *
 * Input: s = "1*"
 * Output: 18
 * Explanation: The encoded message can represent any of the encoded messages "11", "12", "13", "14", "15", "16", "17", "18", or "19".
 * Each of these encoded messages have 2 ways to be decoded (e.g. "11" can be decoded to "AA" or "K").
 * Hence, there are a total of 9 * 2 = 18 ways to decode "1*".
 *
 * Example 3:
 *
 * Input: s = "2*"
 * Output: 15
 * Explanation: The encoded message can represent any of the encoded messages "21", "22", "23", "24", "25", "26", "27", "28", or "29".
 * "21", "22", "23", "24", "25", and "26" have 2 ways of being decoded, but "27", "28", and "29" only have 1 way.
 * Hence, there are a total of (6 * 2) + (3 * 1) = 12 + 3 = 15 ways to decode "2*".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is a digit or '*'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways-ii/
// discuss: https://leetcode.com/problems/decode-ways-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/decode-ways-ii/discuss/1328922/Rust-DP-solution
    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();

        let mut dp = (
            1,
            match s[0] {
                '*' => 9,
                '0' => 0,
                _ => 1,
            },
        );

        for i in 1..s.len() {
            let n = if s[i] == '*' {
                dp.0 * match s[i - 1] {
                    '*' => 15,
                    '1' => 9,
                    '2' => 6,
                    _ => 0,
                } + dp.1 * 9
            } else {
                dp.0 * match s[i - 1] {
                    '*' if s[i] <= '6' => 2,
                    '2' if s[i] <= '6' => 1,
                    '*' | '1' => 1,
                    _ => 0,
                } + if s[i] == '0' { 0 } else { dp.1 }
            };
            dp = (dp.1, n % MOD);
        }

        dp.1 as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0639_example_1() {
        let s = "*".to_string();
        let result = 9;

        assert_eq!(Solution::num_decodings(s), result);
    }

    #[test]
    fn test_0639_example_2() {
        let s = "1*".to_string();
        let result = 18;

        assert_eq!(Solution::num_decodings(s), result);
    }

    #[test]
    fn test_0639_example_3() {
        let s = "2*".to_string();
        let result = 15;

        assert_eq!(Solution::num_decodings(s), result);
    }
}
