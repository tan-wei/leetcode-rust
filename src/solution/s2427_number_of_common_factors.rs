/**
 * [2427] Number of Common Factors
 *
 * Given two positive integers a and b, return the number of common factors of a and b.
 * An integer x is a common factor of a and b if x divides both a and b.
 *  
 * Example 1:
 *
 * Input: a = 12, b = 6
 * Output: 4
 * Explanation: The common factors of 12 and 6 are 1, 2, 3, 6.
 *
 * Example 2:
 *
 * Input: a = 25, b = 30
 * Output: 2
 * Explanation: The common factors of 25 and 30 are 1, 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= a, b <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-common-factors/
// discuss: https://leetcode.com/problems/number-of-common-factors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).filter(|&f| a % f + b % f == 0).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2427_example_1() {
        let a = 12;
        let b = 6;

        let result = 4;

        assert_eq!(Solution::common_factors(a, b), result);
    }

    #[test]
    fn test_2427_example_2() {
        let a = 25;
        let b = 30;

        let result = 2;

        assert_eq!(Solution::common_factors(a, b), result);
    }
}
