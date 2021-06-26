/**
 * [201] Bitwise AND of Numbers Range
 *
 * Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.
 *  
 * Example 1:
 *
 * Input: left = 5, right = 7
 * Output: 4
 *
 * Example 2:
 *
 * Input: left = 0, right = 0
 * Output: 0
 *
 * Example 3:
 *
 * Input: left = 1, right = 2147483647
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= left <= right <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-and-of-numbers-range/
// discuss: https://leetcode.com/problems/bitwise-and-of-numbers-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        match (left, right) {
            (0, 0) => 0,
            (left, right) if left == right => left,
            (left, right) => Self::range_bitwise_and(left >> 1, right >> 1) << 1,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0201_example_1() {
        let left = 5;
        let right = 7;
        let result = 4;

        assert_eq!(Solution::range_bitwise_and(left, right), result);
    }

    #[test]
    fn test_0201_example_2() {
        let left = 0;
        let right = 0;
        let result = 0;

        assert_eq!(Solution::range_bitwise_and(left, right), result);
    }

    #[test]
    fn test_0201_example_3() {
        let left = 1;
        let right = 2147483647;
        let result = 0;

        assert_eq!(Solution::range_bitwise_and(left, right), result);
    }
}
