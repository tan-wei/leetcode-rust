/**
 * [1680] Concatenation of Consecutive Binary Numbers
 *
 * Given an integer n, return the decimal value of the binary string formed by concatenating the binary representations of 1 to n in order, modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 1
 * Explanation: "1" in binary corresponds to the decimal value 1.
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: 27
 * Explanation: In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
 * After concatenating them, we have "11011", which corresponds to the decimal value 27.
 *
 * Example 3:
 *
 * Input: n = 12
 * Output: 505379714
 * Explanation: The concatenation results in "1101110010111011110001001101010111100".
 * The decimal value of that is 118505380540.
 * After modulo 10^9 + 7, the result is 505379714.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
// discuss: https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n as u64).fold(0u64, |o, i| ((o << 64 - i.leading_zeros()) + i) % MOD) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1680_example_1() {
        let n = 1;

        let result = 1;

        assert_eq!(Solution::concatenated_binary(n), result);
    }

    #[test]
    fn test_1680_example_2() {
        let n = 3;

        let result = 27;

        assert_eq!(Solution::concatenated_binary(n), result);
    }

    #[test]
    fn test_1680_example_3() {
        let n = 12;

        let result = 505379714;

        assert_eq!(Solution::concatenated_binary(n), result);
    }
}
