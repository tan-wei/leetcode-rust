/**
 * [2386] Find the K-Sum of an Array
 *
 * You are given an integer array nums and a positive integer k. You can choose any subsequence of the array and sum all of its elements together.
 * We define the K-Sum of the array as the k^th largest subsequence sum that can be obtained (not necessarily distinct).
 * Return the K-Sum of the array.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 * Note that the empty subsequence is considered to have a sum of 0.
 *  
 * Example 1:
 *
 * Input: nums = [2,4,-2], k = 5
 * Output: 2
 * Explanation: All the possible subsequence sums that we can obtain are the following sorted in decreasing order:
 * 6, 4, 4, 2, <u>2</u>, 0, 0, -2.
 * The 5-Sum of the array is 2.
 *
 * Example 2:
 *
 * Input: nums = [1,-2,3,4,-10,12], k = 16
 * Output: 10
 * Explanation: The 16-Sum of the array is 10.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	1 <= k <= min(2000, 2^n)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-k-sum-of-an-array/
// discuss: https://leetcode.com/problems/find-the-k-sum-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2386_example_1() {
        let nums = vec![2, 4, -2];
        let k = 5;

        let result = 2;

        assert_eq!(Solution::k_sum(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2386_example_2() {
        let nums = vec![1, -2, 3, 4, -10, 12];
        let k = 16;

        let result = 10;

        assert_eq!(Solution::k_sum(nums, k), result);
    }
}
