/**
 * [1137] N-th Tribonacci Number
 *
 * The Tribonacci sequence Tn is defined as follows:
 *
 * T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
 *
 * Given n, return the value of Tn.
 *
 *  
 * Example 1:
 *
 *
 * Input: n = 4
 * Output: 4
 * Explanation:
 * T_3 = 0 + 1 + 1 = 2
 * T_4 = 1 + 1 + 2 = 4
 *
 *
 * Example 2:
 *
 *
 * Input: n = 25
 * Output: 1389537
 *
 *
 *  
 * Constraints:
 *
 *
 * 	0 <= n <= 37
 * 	The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 - 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-th-tribonacci-number/
// discuss: https://leetcode.com/problems/n-th-tribonacci-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        (2..n).fold([0, 1, 1], |[a, b, c], _| [b, c, a + b + c])[2.min(n as _)]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1137_example_1() {
        let n = 4;
        let result = 4;

        assert_eq!(Solution::tribonacci(n), result);
    }

    #[test]
    fn test_1137_example_2() {
        let n = 25;
        let result = 1389537;

        assert_eq!(Solution::tribonacci(n), result);
    }
}
