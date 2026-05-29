/**
 * [2485] Find the Pivot Integer
 *
 * Given a positive integer n, find the pivot integer x such that:
 *
 * 	The sum of all elements between 1 and x inclusively equals the sum of all elements between x and n inclusively.
 *
 * Return the pivot integer x. If no such integer exists, return -1. It is guaranteed that there will be at most one pivot index for the given input.
 *  
 * Example 1:
 *
 * Input: n = 8
 * Output: 6
 * Explanation: 6 is the pivot integer since: 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 * Explanation: 1 is the pivot integer since: 1 = 1.
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: -1
 * Explanation: It can be proved that no such integer exist.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-pivot-integer/
// discuss: https://leetcode.com/problems/find-the-pivot-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let result = ((n * (n + 1) / 2) as f32).sqrt();

        if result.fract() > 0.0 {
            -1
        } else {
            result as _
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2485_example_1() {
        let n = 8;

        let result = 6;

        assert_eq!(Solution::pivot_integer(n), result);
    }

    #[test]
    fn test_2485_example_2() {
        let n = 1;

        let result = 1;

        assert_eq!(Solution::pivot_integer(n), result);
    }

    #[test]
    fn test_2485_example_3() {
        let n = 4;

        let result = -1;

        assert_eq!(Solution::pivot_integer(n), result);
    }
}
