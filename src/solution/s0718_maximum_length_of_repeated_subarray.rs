/**
 * [0718] Maximum Length of Repeated Subarray
 *
 * Given two integer arrays nums1 and nums2, return the maximum length of a subarray that appears in both arrays.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
 * Output: 3
 * Explanation: The repeated subarray with maximum length is [3,2,1].
 *
 * Example 2:
 *
 * Input: nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-length-of-repeated-subarray/
// discuss: https://leetcode.com/problems/maximum-length-of-repeated-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();

        if m == 0 || n == 0 {
            return 0;
        }

        // dp[i][j] represents max length of common subarray between nums1[i:] and nums2[j:]
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut result = 0;

        for i in (0..=m - 1).rev() {
            for j in (0..=n - 1).rev() {
                dp[i][j] = if nums1[i] == nums2[j] {
                    1 + dp[i + 1][j + 1]
                } else {
                    0
                };
                result = result.max(dp[i][j]);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0718_example_1() {
        let nums1 = vec![1, 2, 3, 2, 1];
        let nums2 = vec![3, 2, 1, 4, 7];
        let result = 3;

        assert_eq!(Solution::find_length(nums1, nums2), result);
    }

    #[test]
    fn test_0718_example_2() {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![0, 0, 0, 0, 0];
        let result = 5;

        assert_eq!(Solution::find_length(nums1, nums2), result);
    }
}
