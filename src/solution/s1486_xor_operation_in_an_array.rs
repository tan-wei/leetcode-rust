/**
 * [1486] XOR Operation in an Array
 *
 * You are given an integer n and an integer start.
 * Define an array nums where nums[i] = start + 2 * i (0-indexed) and n == nums.length.
 * Return the bitwise XOR of all elements of nums.
 *  
 * Example 1:
 *
 * Input: n = 5, start = 0
 * Output: 8
 * Explanation: Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
 * Where "^" corresponds to bitwise XOR operator.
 *
 * Example 2:
 *
 * Input: n = 4, start = 3
 * Output: 8
 * Explanation: Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	0 <= start <= 1000
 * 	n == nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/xor-operation-in-an-array/
// discuss: https://leetcode.com/problems/xor-operation-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (1..n)
            .into_iter()
            .fold(start, |result, i| result ^ start + 2 * i)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1486_example_1() {
        let n = 5;
        let start = 0;

        let result = 8;

        assert_eq!(Solution::xor_operation(n, start), result);
    }

    #[test]
    fn test_1486_example_2() {
        let n = 4;
        let start = 3;

        let result = 8;

        assert_eq!(Solution::xor_operation(n, start), result);
    }
}
