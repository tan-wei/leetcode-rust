/**
 * [1567] Maximum Length of Subarray With Positive Product
 *
 * Given an array of integers nums, find the maximum length of a subarray where the product of all its elements is positive.
 * A subarray of an array is a consecutive sequence of zero or more values taken out of that array.
 * Return the maximum length of a subarray with positive product.
 *  
 * Example 1:
 *
 * Input: nums = [1,-2,-3,4]
 * Output: 4
 * Explanation: The array nums already has a positive product of 24.
 *
 * Example 2:
 *
 * Input: nums = [0,1,-2,-3,-4]
 * Output: 3
 * Explanation: The longest subarray with positive product is [1,-2,-3] which has a product of 6.
 * Notice that we cannot include 0 in the subarray since that'll make the product 0 which is not positive.
 * Example 3:
 *
 * Input: nums = [-1,-2,-3,0,1]
 * Output: 2
 * Explanation: The longest subarray with positive product is [-1,-2] or [-2,-3].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-length-of-subarray-with-positive-product/
// discuss: https://leetcode.com/problems/maximum-length-of-subarray-with-positive-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut pos = if nums[0] > 0 { 1 } else { 0 };
        let mut result = pos;

        let mut neg = if nums[0] < 0 { 1 } else { 0 };

        for i in 1..n {
            let curr = nums[i];
            if curr == 0 {
                pos = 0;
                neg = 0;
            } else {
                let next_neg = if neg > 0 { neg + 1 } else { 0 };
                let next_pos = pos + 1;
                pos = if curr < 0 { next_neg } else { next_pos };
                neg = if curr < 0 { next_pos } else { next_neg };
            }
            result = std::cmp::max(result, pos);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1567_example_1() {
        let nums = vec![1, -2, -3, 4];

        let result = 4;

        assert_eq!(Solution::get_max_len(nums), result);
    }

    #[test]
    fn test_1567_example_2() {
        let nums = vec![0, 1, -2, -3, -4];

        let result = 3;

        assert_eq!(Solution::get_max_len(nums), result);
    }

    #[test]
    fn test_1567_example_3() {
        let nums = vec![-1, -2, -3, 0, 1];

        let result = 2;

        assert_eq!(Solution::get_max_len(nums), result);
    }
}
