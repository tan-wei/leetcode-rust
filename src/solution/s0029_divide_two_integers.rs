/**
 * [29] Divide Two Integers
 *
 * Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
 * Return the quotient after dividing dividend by divisor.
 * The integer division should truncate toward zero, which means losing its fractional part. For example, truncate(8.345) = 8 and truncate(-2.7335) = -2.
 * Note:
 *
 * 	Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For this problem, assume that your function returns 2^31 - 1 when the division result overflows.
 *
 *  
 * Example 1:
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Explanation: 10/3 = truncate(3.33333..) = 3.
 *
 * Example 2:
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Explanation: 7/-3 = truncate(-2.33333..) = -2.
 *
 * Example 3:
 *
 * Input: dividend = 0, divisor = 1
 * Output: 0
 *
 * Example 4:
 *
 * Input: dividend = 1, divisor = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= dividend, divisor <= 2^31 - 1
 * 	divisor != 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        match dividend.checked_div(divisor) {
            Some(i) => i,
            None => std::i32::MAX,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0029_example_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_0029_example_2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_0029_example_3() {
        assert_eq!(Solution::divide(0, 1), 0);
    }

    #[test]
    fn test_0029_example_4() {
        assert_eq!(Solution::divide(1, 1), 1);
    }
}
