/**
 * [1458] Max Dot Product of Two Subsequences
 *
 * Given two arrays nums1 and <font face="monospace">nums2</font><font face="monospace">.</font>
 * Return the maximum dot product between non-empty subsequences of nums1 and nums2 with the same length.
 * A subsequence of a array is a new array which is formed from the original array by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, [2,3,5] is a subsequence of [1,2,3,4,5] while [1,5,3] is not).
 *  
 * Example 1:
 *
 * Input: nums1 = [2,1,-2,5], nums2 = [3,0,-6]
 * Output: 18
 * Explanation: Take subsequence [2,-2] from nums1 and subsequence [3,-6] from nums2.
 * Their dot product is (2*3 + (-2)*(-6)) = 18.
 * Example 2:
 *
 * Input: nums1 = [3,-2], nums2 = [2,-6,7]
 * Output: 21
 * Explanation: Take subsequence [3] from nums1 and subsequence [7] from nums2.
 * Their dot product is (3*7) = 21.
 * Example 3:
 *
 * Input: nums1 = [-1,-1], nums2 = [1,1]
 * Output: -1
 * Explanation: Take subsequence [-1] from nums1 and subsequence [1] from nums2.
 * Their dot product is -1.
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 500
 * 	-1000 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-dot-product-of-two-subsequences/
// discuss: https://leetcode.com/problems/max-dot-product-of-two-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/max-dot-product-of-two-subsequences/solutions/4145799/rust-2-d-dynamic-programming/
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m]; n];

        dp[0][0] = nums1[0] * nums2[0];
        let mut result = dp[0][0];

        for i in 1..n {
            dp[i][0] = dp[i - 1][0].max(nums1[i] * nums2[0]);
            result = result.max(dp[i][0]);
        }

        for j in 1..m {
            dp[0][j] = dp[0][j - 1].max(nums1[0] * nums2[j]);
            result = result.max(dp[0][j]);
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i - 1][j]
                    .max(dp[i][j - 1])
                    .max(dp[i - 1][j - 1] + nums1[i] * nums2[j])
                    .max(nums1[i] * nums2[j]);
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
    fn test_1458_example_1() {
        let nums1 = vec![2, 1, -2, 5];
        let nums2 = vec![3, 0, -6];

        let result = 18;

        assert_eq!(Solution::max_dot_product(nums1, nums2), result);
    }

    #[test]
    fn test_1458_example_2() {
        let nums1 = vec![3, -2];
        let nums2 = vec![2, -6, 7];

        let result = 21;

        assert_eq!(Solution::max_dot_product(nums1, nums2), result);
    }

    #[test]
    fn test_1458_example_3() {
        let nums1 = vec![-1, -1];
        let nums2 = vec![1, 1];

        let result = -1;

        assert_eq!(Solution::max_dot_product(nums1, nums2), result);
    }
}
