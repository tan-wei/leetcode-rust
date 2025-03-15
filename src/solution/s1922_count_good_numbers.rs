/**
 * [1922] Count Good Numbers
 *
 * A digit string is good if the digits (0-indexed) at even indices are even and the digits at odd indices are prime (2, 3, 5, or 7).
 *
 * 	For example, "2582" is good because the digits (2 and 8) at even positions are even and the digits (5 and 2) at odd positions are prime. However, "3245" is not good because 3 is at an even index but is not even.
 *
 * Given an integer n, return the total number of good digit strings of length n. Since the answer may be large, return it modulo 10^9 + 7.
 * A digit string is a string consisting of digits 0 through 9 that may contain leading zeros.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 5
 * Explanation: The good numbers of length 1 are "0", "2", "4", "6", "8".
 *
 * Example 2:
 *
 * Input: n = 4
 * Output: 400
 *
 * Example 3:
 *
 * Input: n = 50
 * Output: 564908303
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-good-numbers/
// discuss: https://leetcode.com/problems/count-good-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1922_example_1() {
        let n = 1;

        let result = 5;

        assert_eq!(Solution::count_good_numbers(n), result);
    }

    #[test]
    #[ignore]
    fn test_1922_example_2() {
        let n = 4;

        let result = 400;

        assert_eq!(Solution::count_good_numbers(n), result);
    }

    #[test]
    #[ignore]
    fn test_1922_example_3() {
        let n = 50;

        let result = 564908303;

        assert_eq!(Solution::count_good_numbers(n), result);
    }
}
