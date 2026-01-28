/**
 * [2333] Minimum Sum of Squared Difference
 *
 * You are given two positive 0-indexed integer arrays nums1 and nums2, both of length n.
 * The sum of squared difference of arrays nums1 and nums2 is defined as the sum of (nums1[i] - nums2[i])^2 for each 0 <= i < n.
 * You are also given two positive integers k1 and k2. You can modify any of the elements of nums1 by +1 or -1 at most k1 times. Similarly, you can modify any of the elements of nums2 by +1 or -1 at most k2 times.
 * Return the minimum sum of squared difference after modifying array nums1 at most k1 times and modifying array nums2 at most k2 times.
 * Note: You are allowed to modify the array elements to become negative integers.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2,3,4], nums2 = [2,10,20,19], k1 = 0, k2 = 0
 * Output: 579
 * Explanation: The elements in nums1 and nums2 cannot be modified because k1 = 0 and k2 = 0.
 * The sum of square difference will be: (1 - 2)^2 + (2 - 10)^2 + (3 - 20)^2 + (4 - 19)^2 = 579.
 *
 * Example 2:
 *
 * Input: nums1 = [1,4,10,12], nums2 = [5,8,6,9], k1 = 1, k2 = 1
 * Output: 43
 * Explanation: One way to obtain the minimum sum of square difference is:
 * - Increase nums1[0] once.
 * - Increase nums2[2] once.
 * The minimum of the sum of square difference will be:
 * (2 - 5)^2 + (4 - 8)^2 + (10 - 7)^2 + (12 - 9)^2 = 43.
 * Note that, there are other ways to obtain the minimum of the sum of square difference, but there is no way to obtain a sum smaller than 43.
 *  
 * Constraints:
 *
 * 	n == nums1.length == nums2.length
 * 	1 <= n <= 10^5
 * 	0 <= nums1[i], nums2[i] <= 10^5
 * 	0 <= k1, k2 <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-sum-of-squared-difference/
// discuss: https://leetcode.com/problems/minimum-sum-of-squared-difference/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2333_example_1() {
        let nums1 = vec![1, 2, 3, 4];
        let nums2 = vec![2, 10, 20, 19];
        let k1 = 0;
        let k2 = 0;

        let result = 579;

        assert_eq!(Solution::min_sum_square_diff(nums1, nums2, k1, k2), result);
    }

    #[test]
    #[ignore]
    fn test_2333_example_2() {
        let nums1 = vec![1, 4, 10, 12];
        let nums2 = vec![5, 8, 6, 9];
        let k1 = 1;
        let k2 = 1;

        let result = 43;

        assert_eq!(Solution::min_sum_square_diff(nums1, nums2, k1, k2), result);
    }
}
