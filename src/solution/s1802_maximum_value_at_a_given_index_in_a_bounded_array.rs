/**
 * [1802] Maximum Value at a Given Index in a Bounded Array
 *
 * You are given three positive integers: n, index, and maxSum. You want to construct an array nums (0-indexed) that satisfies the following conditions:
 *
 * 	nums.length == n
 * 	nums[i] is a positive integer where 0 <= i < n.
 * 	abs(nums[i] - nums[i+1]) <= 1 where 0 <= i < n-1.
 * 	The sum of all the elements of nums does not exceed maxSum.
 * 	nums[index] is maximized.
 *
 * Return nums[index] of the constructed array.
 * Note that abs(x) equals x if x >= 0, and -x otherwise.
 *  
 * Example 1:
 *
 * Input: n = 4, index = 2,  maxSum = 6
 * Output: 2
 * Explanation: nums = [1,2,<u>2</u>,1] is one array that satisfies all the conditions.
 * There are no arrays that satisfy all the conditions and have nums[2] == 3, so 2 is the maximum nums[2].
 *
 * Example 2:
 *
 * Input: n = 6, index = 1,  maxSum = 10
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= maxSum <= 10^9
 * 	0 <= index < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/
// discuss: https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1802_example_1() {
        let n = 4;
        let index = 2;
        let max_sum = 6;

        let result = 2;

        assert_eq!(Solution::max_value(n, index, max_sum), result);
    }

    #[test]
    #[ignore]
    fn test_1802_example_2() {
        let n = 6;
        let index = 1;
        let max_sum = 10;

        let result = 13;

        assert_eq!(Solution::max_value(n, index, max_sum), result);
    }
}
