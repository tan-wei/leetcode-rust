/**
 * [2571] Minimum Operations to Reduce an Integer to 0
 *
 * You are given a positive integer n, you can do the following operation any number of times:
 *
 * 	Add or subtract a power of 2 from n.
 *
 * Return the minimum number of operations to make n equal to 0.
 * A number x is power of 2 if x == 2^i where i >= 0.
 *  
 * Example 1:
 *
 * Input: n = 39
 * Output: 3
 * Explanation: We can do the following operations:
 * - Add 2^0 = 1 to n, so now n = 40.
 * - Subtract 2^3 = 8 from n, so now n = 32.
 * - Subtract 2^5 = 32 from n, so now n = 0.
 * It can be shown that 3 is the minimum number of operations we need to make n equal to 0.
 *
 * Example 2:
 *
 * Input: n = 54
 * Output: 3
 * Explanation: We can do the following operations:
 * - Add 2^1 = 2 to n, so now n = 56.
 * - Add 2^3 = 8 to n, so now n = 64.
 * - Subtract 2^6 = 64 from n, so now n = 0.
 * So the minimum number of operations is 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-reduce-an-integer-to-0/
// discuss: https://leetcode.com/problems/minimum-operations-to-reduce-an-integer-to-0/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2571_example_1() {
        let n = 39;

        let result = 3;

        assert_eq!(Solution::min_operations(n), result);
    }

    #[test]
    #[ignore]
    fn test_2571_example_2() {
        let n = 54;

        let result = 3;

        assert_eq!(Solution::min_operations(n), result);
    }
}
