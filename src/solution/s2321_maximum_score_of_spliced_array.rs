/**
 * [2321] Maximum Score Of Spliced Array
 *
 * You are given two 0-indexed integer arrays nums1 and nums2, both of length n.
 * You can choose two integers left and right where 0 <= left <= right < n and swap the subarray nums1[left...right] with the subarray nums2[left...right].
 *
 * 	For example, if nums1 = [1,2,3,4,5] and nums2 = [11,12,13,14,15] and you choose left = 1 and right = 2, nums1 becomes [1,<u>12,13</u>,4,5] and nums2 becomes [11,<u>2,3</u>,14,15].
 *
 * You may choose to apply the mentioned operation once or not do anything.
 * The score of the arrays is the maximum of sum(nums1) and sum(nums2), where sum(arr) is the sum of all the elements in the array arr.
 * Return the maximum possible score.
 * A subarray is a contiguous sequence of elements within an array. arr[left...right] denotes the subarray that contains the elements of nums between indices left and right (inclusive).
 *  
 * Example 1:
 *
 * Input: nums1 = [60,60,60], nums2 = [10,90,10]
 * Output: 210
 * Explanation: Choosing left = 1 and right = 1, we have nums1 = [60,<u>90</u>,60] and nums2 = [10,<u>60</u>,10].
 * The score is max(sum(nums1), sum(nums2)) = max(210, 80) = 210.
 * Example 2:
 *
 * Input: nums1 = [20,40,20,70,30], nums2 = [50,20,50,40,20]
 * Output: 220
 * Explanation: Choosing left = 3, right = 4, we have nums1 = [20,40,20,<u>40,20</u>] and nums2 = [50,20,50,<u>70,30</u>].
 * The score is max(sum(nums1), sum(nums2)) = max(140, 220) = 220.
 *
 * Example 3:
 *
 * Input: nums1 = [7,11,13], nums2 = [1,1,1]
 * Output: 31
 * Explanation: We choose not to swap any subarray.
 * The score is max(sum(nums1), sum(nums2)) = max(31, 3) = 31.
 *
 *  
 * Constraints:
 *
 * 	n == nums1.length == nums2.length
 * 	1 <= n <= 10^5
 * 	1 <= nums1[i], nums2[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-of-spliced-array/
// discuss: https://leetcode.com/problems/maximum-score-of-spliced-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2321_example_1() {
        let nums1 = vec![60, 60, 60];
        let nums2 = vec![10, 90, 10];

        let result = 210;

        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), result);
    }

    #[test]
    #[ignore]
    fn test_2321_example_2() {
        let nums1 = vec![7, 11, 13];
        let nums2 = vec![1, 1, 1];

        let result = 31;

        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), result);
    }
}
