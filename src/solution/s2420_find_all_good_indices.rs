/**
 * [2420] Find All Good Indices
 *
 * You are given a 0-indexed integer array nums of size n and a positive integer k.
 * We call an index i in the range k <= i < n - k good if the following conditions are satisfied:
 *
 * 	The k elements that are just before the index i are in non-increasing order.
 * 	The k elements that are just after the index i are in non-decreasing order.
 *
 * Return an array of all good indices sorted in increasing order.
 *  
 * Example 1:
 *
 * Input: nums = [2,1,1,1,3,4,1], k = 2
 * Output: [2,3]
 * Explanation: There are two good indices in the array:
 * - Index 2. The subarray [2,1] is in non-increasing order, and the subarray [1,3] is in non-decreasing order.
 * - Index 3. The subarray [1,1] is in non-increasing order, and the subarray [3,4] is in non-decreasing order.
 * Note that the index 4 is not good because [4,1] is not non-decreasing.
 * Example 2:
 *
 * Input: nums = [2,1,1,2], k = 2
 * Output: []
 * Explanation: There are no good indices in this array.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	3 <= n <= 10^5
 * 	1 <= nums[i] <= 10^6
 * 	1 <= k <= n / 2
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-good-indices/
// discuss: https://leetcode.com/problems/find-all-good-indices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2420_example_1() {
        let nums = vec![2, 1, 1, 1, 3, 4, 1];
        let k = 2;

        let result = vec![2, 3];

        assert_eq!(Solution::good_indices(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2420_example_2() {
        let nums = vec![2, 1, 1, 2];
        let k = 2;

        let result: Vec<i32> = vec![];

        assert_eq!(Solution::good_indices(nums, k), result);
    }
}
