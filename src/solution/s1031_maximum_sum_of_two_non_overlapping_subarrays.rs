/**
 * [1031] Maximum Sum of Two Non-Overlapping Subarrays
 *
 * Given an integer array nums and two integers firstLen and secondLen, return the maximum sum of elements in two non-overlapping subarrays with lengths firstLen and secondLen.
 * The array with length firstLen could occur before or after the array with length secondLen, but they have to be non-overlapping.
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
 * Output: 20
 * Explanation: One choice of subarrays is [9] with length 1, and [6,5] with length 2.
 *
 * Example 2:
 *
 * Input: nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
 * Output: 29
 * Explanation: One choice of subarrays is [3,8,1] with length 3, and [8,9] with length 2.
 *
 * Example 3:
 *
 * Input: nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
 * Output: 31
 * Explanation: One choice of subarrays is [5,6,0,9] with length 4, and [0,3,8] with length 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= firstLen, secondLen <= 1000
 * 	2 <= firstLen + secondLen <= 1000
 * 	firstLen + secondLen <= nums.length <= 1000
 * 	0 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
// discuss: https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut nums = nums;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        let (mut result, mut first_max, mut second_max) = (
            nums[first_len + second_len - 1],
            nums[first_len - 1],
            nums[second_len - 1],
        );

        for i in (first_len + second_len)..nums.len() {
            first_max = first_max.max(nums[i - second_len] - nums[i - first_len - second_len]);
            second_max = second_max.max(nums[i - first_len] - nums[i - first_len - second_len]);
            result = result
                .max(first_max + nums[i] - nums[i - second_len])
                .max(second_max + nums[i] - nums[i - first_len]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1031_example_1() {
        let nums = vec![0, 6, 5, 2, 2, 5, 1, 9, 4];
        let first_len = 1;
        let second_len = 2;
        let result = 20;

        assert_eq!(
            Solution::max_sum_two_no_overlap(nums, first_len, second_len),
            result
        );
    }

    #[test]
    fn test_1031_example_2() {
        let nums = vec![3, 8, 1, 3, 2, 1, 8, 9, 0];
        let first_len = 3;
        let second_len = 2;
        let result = 29;

        assert_eq!(
            Solution::max_sum_two_no_overlap(nums, first_len, second_len),
            result
        );
    }

    #[test]
    fn test_1031_example_3() {
        let nums = vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
        let first_len = 4;
        let second_len = 3;
        let result = 31;

        assert_eq!(
            Solution::max_sum_two_no_overlap(nums, first_len, second_len),
            result
        );
    }
}
