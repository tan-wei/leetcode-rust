/**
 * [2488] Count Subarrays With Median K
 *
 * You are given an array nums of size n consisting of distinct integers from 1 to n and a positive integer k.
 * Return the number of non-empty subarrays in nums that have a median equal to k.
 * Note:
 *
 * 	The median of an array is the middle element after sorting the array in ascending order. If the array is of even length, the median is the left middle element.
 *
 * 		For example, the median of [2,3,1,4] is 2, and the median of [8,4,3,5,1] is 4.
 *
 *
 * 	A subarray is a contiguous part of an array.
 *
 *  
 * Example 1:
 *
 * Input: nums = [3,2,1,4,5], k = 4
 * Output: 3
 * Explanation: The subarrays that have a median equal to 4 are: [4], [4,5] and [1,4,5].
 *
 * Example 2:
 *
 * Input: nums = [2,3,1], k = 3
 * Output: 1
 * Explanation: [3] is the only subarray that has a median equal to 3.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i], k <= n
 * 	The integers in nums are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-with-median-k/
// discuss: https://leetcode.com/problems/count-subarrays-with-median-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2488_example_1() {
        let nums = vec![3, 2, 1, 4, 5];
        let k = 4;

        let result = 3;

        assert_eq!(Solution::count_subarrays(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2488_example_2() {
        let nums = vec![2, 3, 1];
        let k = 3;

        let result = 1;

        assert_eq!(Solution::count_subarrays(nums, k), result);
    }
}
