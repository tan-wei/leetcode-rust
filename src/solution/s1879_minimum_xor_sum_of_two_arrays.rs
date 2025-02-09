/**
 * [1879] Minimum XOR Sum of Two Arrays
 *
 * You are given two integer arrays nums1 and nums2 of length n.
 * The XOR sum of the two integer arrays is (nums1[0] XOR nums2[0]) + (nums1[1] XOR nums2[1]) + ... + (nums1[n - 1] XOR nums2[n - 1]) (0-indexed).
 *
 * 	For example, the XOR sum of [1,2,3] and [3,2,1] is equal to (1 XOR 3) + (2 XOR 2) + (3 XOR 1) = 2 + 0 + 2 = 4.
 *
 * Rearrange the elements of nums2 such that the resulting XOR sum is minimized.
 * Return the XOR sum after the rearrangement.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2], nums2 = [2,3]
 * Output: 2
 * Explanation: Rearrange nums2 so that it becomes [3,2].
 * The XOR sum is (1 XOR 3) + (2 XOR 2) = 2 + 0 = 2.
 * Example 2:
 *
 * Input: nums1 = [1,0,3], nums2 = [5,3,4]
 * Output: 8
 * Explanation: Rearrange nums2 so that it becomes [5,4,3].
 * The XOR sum is (1 XOR 5) + (0 XOR 4) + (3 XOR 3) = 4 + 4 + 0 = 8.
 *
 *  
 * Constraints:
 *
 * 	n == nums1.length
 * 	n == nums2.length
 * 	1 <= n <= 14
 * 	0 <= nums1[i], nums2[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-xor-sum-of-two-arrays/
// discuss: https://leetcode.com/problems/minimum-xor-sum-of-two-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1879_example_1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![2, 3];

        let result = 2;

        assert_eq!(Solution::minimum_xor_sum(nums1, nums2), result);
    }

    #[test]
    #[ignore]
    fn test_1879_example_2() {
        let nums1 = vec![1, 0, 3];
        let nums2 = vec![5, 3, 4];

        let result = 8;

        assert_eq!(Solution::minimum_xor_sum(nums1, nums2), result);
    }
}
