/**
 * [2413] Smallest Even Multiple
 *
 * Given a positive integer n, return the smallest positive integer that is a multiple of both 2 and n.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 10
 * Explanation: The smallest multiple of both 5 and 2 is 10.
 *
 * Example 2:
 *
 * Input: n = 6
 * Output: 6
 * Explanation: The smallest multiple of both 6 and 2 is 6. Note that a number is a multiple of itself.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 150
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-even-multiple/
// discuss: https://leetcode.com/problems/smallest-even-multiple/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        n << (n & 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2413_example_1() {
        let n = 5;

        let result = 10;

        assert_eq!(Solution::smallest_even_multiple(n), result);
    }

    #[test]
    fn test_2413_example_2() {
        let n = 6;

        let result = 6;

        assert_eq!(Solution::smallest_even_multiple(n), result);
    }
}
