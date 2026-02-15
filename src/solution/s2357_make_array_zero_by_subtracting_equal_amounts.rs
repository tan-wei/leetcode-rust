/**
 * [2357] Make Array Zero by Subtracting Equal Amounts
 *
 * You are given a non-negative integer array nums. In one operation, you must:
 *
 * 	Choose a positive integer x such that x is less than or equal to the smallest non-zero element in nums.
 * 	Subtract x from every positive element in nums.
 *
 * Return the minimum number of operations to make every element in nums equal to 0.
 *  
 * Example 1:
 *
 * Input: nums = [1,5,0,3,5]
 * Output: 3
 * Explanation:
 * In the first operation, choose x = 1. Now, nums = [0,4,0,2,4].
 * In the second operation, choose x = 2. Now, nums = [0,2,0,0,2].
 * In the third operation, choose x = 2. Now, nums = [0,0,0,0,0].
 *
 * Example 2:
 *
 * Input: nums = [0]
 * Output: 0
 * Explanation: Each element in nums is already 0 so no operations are needed.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/
// discuss: https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.chunk_by(|a, b| *a == *b)
            .skip((nums[0] == 0) as usize)
            .count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2357_example_1() {
        let nums = vec![1, 5, 0, 3, 5];

        let result = 3;

        assert_eq!(Solution::minimum_operations(nums), result);
    }

    #[test]
    fn test_2357_example_2() {
        let nums = vec![0];

        let result = 0;

        assert_eq!(Solution::minimum_operations(nums), result);
    }
}
