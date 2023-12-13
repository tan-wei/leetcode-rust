/**
 * [1416] Restore The Array
 *
 * A program was supposed to print an array of integers. The program forgot to print whitespaces and the array is printed as a string of digits s and all we know is that all integers in the array were in the range [1, k] and there are no leading zeros in the array.
 * Given the string s and the integer k, return the number of the possible arrays that can be printed as s using the mentioned program. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: s = "1000", k = 10000
 * Output: 1
 * Explanation: The only possible array is [1000]
 *
 * Example 2:
 *
 * Input: s = "1000", k = 10
 * Output: 0
 * Explanation: There cannot be an array that was printed this way and has all integer >= 1 and <= 10.
 *
 * Example 3:
 *
 * Input: s = "1317", k = 2000
 * Output: 8
 * Explanation: Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only digits and does not contain leading zeros.
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/restore-the-array/
// discuss: https://leetcode.com/problems/restore-the-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let k_10 = k / 10;
        let mut dp = vec![0; n + 1];

        dp[n] = 1;

        let bytes = s.as_bytes();
        for (i, c) in bytes.iter().copied().enumerate().rev() {
            let mut num_ways = 0;
            if c != b'0' {
                let mut num = 0;
                for idx in i..n {
                    if num <= k_10 {
                        let d = (bytes[idx] - b'0') as i32;
                        num = num * 10 + d;
                        if num <= k {
                            num_ways = (num_ways + dp[idx + 1]) % MOD;
                            continue;
                        }
                    }
                    break;
                }
            }
            dp[i] = num_ways;
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1416_example_1() {
        let s = "1000".to_string();
        let k = 10000;

        let result = 1;

        assert_eq!(Solution::number_of_arrays(s, k), result);
    }

    #[test]
    fn test_1416_example_2() {
        let s = "1000".to_string();
        let k = 10;

        let result = 0;

        assert_eq!(Solution::number_of_arrays(s, k), result);
    }

    #[test]
    fn test_1416_example_3() {
        let s = "1317".to_string();
        let k = 2000;

        let result = 8;

        assert_eq!(Solution::number_of_arrays(s, k), result);
    }
}
