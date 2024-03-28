/**
 * [1537] Get the Maximum Score
 *
 * You are given two sorted arrays of distinct integers nums1 and nums2.
 * A valid path is defined as follows:
 *
 * 	Choose array nums1 or nums2 to traverse (from index-0).
 * 	Traverse the current array from left to right.
 * 	If you are reading any value that is present in nums1 and nums2 you are allowed to change your path to the other array. (Only one repeated value is considered in the valid path).
 *
 * The score is defined as the sum of unique values in a valid path.
 * Return the maximum score you can obtain of all possible valid paths. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/16/sample_1_1893.png" style="width: 500px; height: 151px;" />
 * Input: nums1 = [2,4,5,8,10], nums2 = [4,6,8,9]
 * Output: 30
 * Explanation: Valid paths:
 * [2,4,5,8,10], [2,4,5,8,9], [2,4,6,8,9], [2,4,6,8,10],  (starting from nums1)
 * [4,6,8,9], [4,5,8,10], [4,5,8,9], [4,6,8,10]    (starting from nums2)
 * The maximum is obtained with the path in green [2,4,6,8,10].
 *
 * Example 2:
 *
 * Input: nums1 = [1,3,5,7,9], nums2 = [3,5,100]
 * Output: 109
 * Explanation: Maximum sum is obtained with the path [1,3,5,100].
 *
 * Example 3:
 *
 * Input: nums1 = [1,2,3,4,5], nums2 = [6,7,8,9,10]
 * Output: 40
 * Explanation: There are no common elements between nums1 and nums2.
 * Maximum sum is obtained with the path [6,7,8,9,10].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 10^5
 * 	1 <= nums1[i], nums2[i] <= 10^7
 * 	nums1 and nums2 are strictly increasing.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/get-the-maximum-score/
// discuss: https://leetcode.com/problems/get-the-maximum-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/get-the-maximum-score/solutions/3167241/just-a-runnable-solution/
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j, mut a, mut b, n, m) = (0, 0, 0, 0, nums1.len(), nums2.len());
        let mod_num = 1_000_000_007;

        while i < n || j < m {
            if i < n && (j == m || nums1[i] < nums2[j]) {
                a += nums1[i] as i64;
                i += 1;
            } else if j < m && (i == n || nums1[i] > nums2[j]) {
                b += nums2[j] as i64;
                j += 1;
            } else {
                b = a.max(b) + nums1[i] as i64;
                a = b;
                i += 1;
                j += 1;
            }
        }

        (a.max(b) % mod_num) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1537_example_1() {
        let nums1 = vec![2, 4, 5, 8, 10];
        let nums2 = vec![4, 6, 8, 9];

        let result = 30;

        assert_eq!(Solution::max_sum(nums1, nums2), result);
    }

    #[test]
    fn test_1537_example_2() {
        let nums1 = vec![1, 3, 5, 7, 9];
        let nums2 = vec![3, 5, 100];

        let result = 109;

        assert_eq!(Solution::max_sum(nums1, nums2), result);
    }

    #[test]
    fn test_1537_example_3() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![6, 7, 8, 9, 10];

        let result = 40;

        assert_eq!(Solution::max_sum(nums1, nums2), result);
    }
}
