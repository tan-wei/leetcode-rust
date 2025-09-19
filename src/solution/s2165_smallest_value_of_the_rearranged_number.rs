/**
 * [2165] Smallest Value of the Rearranged Number
 *
 * You are given an integer num. Rearrange the digits of num such that its value is minimized and it does not contain any leading zeros.
 * Return the rearranged number with minimal value.
 * Note that the sign of the number does not change after rearranging the digits.
 *  
 * Example 1:
 *
 * Input: num = 310
 * Output: 103
 * Explanation: The possible arrangements for the digits of 310 are 013, 031, 103, 130, 301, 310.
 * The arrangement with the smallest value that does not contain any leading zeros is 103.
 *
 * Example 2:
 *
 * Input: num = -7605
 * Output: -7650
 * Explanation: Some possible arrangements for the digits of -7605 are -7650, -6705, -5076, -0567.
 * The arrangement with the smallest value that does not contain any leading zeros is -7650.
 *
 *  
 * Constraints:
 *
 * 	-10^15 <= num <= 10^15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-value-of-the-rearranged-number/
// discuss: https://leetcode.com/problems/smallest-value-of-the-rearranged-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2165_example_1() {
        let num = 310;

        let result = 103;

        assert_eq!(Solution::smallest_number(num), result);
    }

    #[test]
    #[ignore]
    fn test_2165_example_2() {
        let num = -7605;

        let result = -7650;

        assert_eq!(Solution::smallest_number(num), result);
    }
}
