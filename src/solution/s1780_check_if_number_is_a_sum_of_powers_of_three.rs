/**
 * [1780] Check if Number is a Sum of Powers of Three
 *
 * Given an integer n, return true if it is possible to represent n as the sum of distinct powers of three. Otherwise, return false.
 * An integer y is a power of three if there exists an integer x such that y == 3^x.
 *  
 * Example 1:
 *
 * Input: n = 12
 * Output: true
 * Explanation: 12 = 3^1 + 3^2
 *
 * Example 2:
 *
 * Input: n = 91
 * Output: true
 * Explanation: 91 = 3^0 + 3^2 + 3^4
 *
 * Example 3:
 *
 * Input: n = 21
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
// discuss: https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut temp = n;

        while temp > 0 {
            if temp % 3 == 2 {
                return false;
            }
            temp /= 3;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1780_example_1() {
        let n = 12;

        let result = true;

        assert_eq!(Solution::check_powers_of_three(n), result);
    }

    #[test]
    fn test_1780_example_2() {
        let n = 91;

        let result = true;

        assert_eq!(Solution::check_powers_of_three(n), result);
    }

    #[test]
    fn test_1780_example_3() {
        let n = 21;

        let result = false;

        assert_eq!(Solution::check_powers_of_three(n), result);
    }
}
