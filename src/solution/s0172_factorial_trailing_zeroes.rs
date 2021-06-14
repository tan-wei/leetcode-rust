/**
 * [172] Factorial Trailing Zeroes
 *
 * Given an integer n, return the number of trailing zeroes in n!.
 * Follow up: Could you write a solution that works in logarithmic time complexity?
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 0
 * Explanation: 3! = 6, no trailing zero.
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: 1
 * Explanation: 5! = 120, one trailing zero.
 *
 * Example 3:
 *
 * Input: n = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/factorial-trailing-zeroes/
// discuss: https://leetcode.com/problems/factorial-trailing-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        (5..=n)
            .filter(|x| 1_220_703_125 % x == 0)
            .fold(0, |acc, cur| acc + n / cur)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0172_example_1() {
        let n = 3;
        let result = 0;

        assert_eq!(Solution::trailing_zeroes(n), result);
    }

    #[test]
    fn test_0172_example_2() {
        let n = 5;
        let result = 1;

        assert_eq!(Solution::trailing_zeroes(n), result);
    }

    #[test]
    fn test_0172_example_3() {
        let n = 0;
        let result = 0;

        assert_eq!(Solution::trailing_zeroes(n), result);
    }
}
