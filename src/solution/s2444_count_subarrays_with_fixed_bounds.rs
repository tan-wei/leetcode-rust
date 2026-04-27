/**
 * [2444] Count Subarrays With Fixed Bounds
 *
 * You are given an integer array nums and two integers minK and maxK.
 * A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
 *
 * 	The minimum value in the subarray is equal to minK.
 * 	The maximum value in the subarray is equal to maxK.
 *
 * Return the number of fixed-bound subarrays.
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
 * Output: 2
 * Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,1], minK = 1, maxK = 1
 * Output: 10
 * Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	1 <= nums[i], minK, maxK <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
// discuss: https://leetcode.com/problems/count-subarrays-with-fixed-bounds/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2444_example_1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;

        let result = 2;

        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), result)
    }

    #[test]
    #[ignore]
    fn test_2444_example_2() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;

        let result = 10;

        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), result)
    }
}
