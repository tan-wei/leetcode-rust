/**
 * [238] Product of Array Except Self
 *
 * Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 * You must write an algorithm that runs in O(n) time and without using the division operation.
 *  
 * Example 1:
 * Input: nums = [1,2,3,4]
 * Output: [24,12,8,6]
 * Example 2:
 * Input: nums = [-1,1,0,-3,3]
 * Output: [0,0,9,0,0]
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	-30 <= nums[i] <= 30
 * 	The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 *
 *  
 * Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/product-of-array-except-self/
// discuss: https://leetcode.com/problems/product-of-array-except-self/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        (0..nums.len()).fold(1i32, |mut sum, i| {
            result[i] = sum;
            sum *= nums[i];
            sum
        });
        (0..nums.len()).rev().fold(1i32, |mut sum, i| {
            result[i] *= sum;
            sum *= nums[i];
            sum
        });
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0238_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = vec![24, 12, 8, 6];

        assert_eq!(Solution::product_except_self(nums), result);
    }

    #[test]
    fn test_0238_example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = vec![0, 0, 9, 0, 0];

        assert_eq!(Solution::product_except_self(nums), result);
    }
}
