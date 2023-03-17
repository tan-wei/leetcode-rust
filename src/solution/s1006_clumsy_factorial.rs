/**
 * [1006] Clumsy Factorial
 *
 * The factorial of a positive integer n is the product of all positive integers less than or equal to n.
 *
 * 	For example, factorial(10) = 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1.
 *
 * We make a clumsy factorial using the integers in decreasing order by swapping out the multiply operations for a fixed rotation of operations with multiply '*', divide '/', add '+', and subtract '-' in this order.
 *
 * 	For example, clumsy(10) = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1.
 *
 * However, these operations are still applied using the usual order of operations of arithmetic. We do all multiplication and division steps before any addition or subtraction steps, and multiplication and division steps are processed left to right.
 * Additionally, the division that we use is floor division such that 10 * 9 / 8 = 90 / 8 = 11.
 * Given an integer n, return the clumsy factorial of n.
 *  
 * Example 1:
 *
 * Input: n = 4
 * Output: 7
 * Explanation: 7 = 4 * 3 / 2 + 1
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 12
 * Explanation: 12 = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/clumsy-factorial/
// discuss: https://leetcode.com/problems/clumsy-factorial/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            3 => 6,
            4 => 7,
            n if n % 4 == 1 || n % 4 == 2 => n + 2,
            n if n % 4 == 3 => n - 1,
            n @ _ => n + 1,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1006_example_1() {
        let n = 4;
        let result = 7;

        assert_eq!(Solution::clumsy(n), result);
    }

    #[test]
    fn test_1006_example_2() {
        let n = 10;
        let result = 12;

        assert_eq!(Solution::clumsy(n), result);
    }
}
