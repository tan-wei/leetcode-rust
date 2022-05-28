/**
 * [0693] Binary Number with Alternating Bits
 *
 * Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: true
 * Explanation: The binary representation of 5 is: 101
 *
 * Example 2:
 *
 * Input: n = 7
 * Output: false
 * Explanation: The binary representation of 7 is: 111.
 * Example 3:
 *
 * Input: n = 11
 * Output: false
 * Explanation: The binary representation of 11 is: 1011.
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-number-with-alternating-bits/
// discuss: https://leetcode.com/problems/binary-number-with-alternating-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        n <= 2 || n ^ (n >> 1) == ((n as usize).next_power_of_two() - 1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0693_example_1() {
        let n = 5;
        let result = true;

        assert_eq!(Solution::has_alternating_bits(n), result);
    }

    #[test]
    fn test_0693_example_2() {
        let n = 7;
        let result = false;

        assert_eq!(Solution::has_alternating_bits(n), result);
    }

    #[test]
    fn test_0693_example_3() {
        let n = 11;
        let result = false;

        assert_eq!(Solution::has_alternating_bits(n), result);
    }
}
