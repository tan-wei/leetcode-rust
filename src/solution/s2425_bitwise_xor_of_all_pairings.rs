/**
 * [2425] Bitwise XOR of All Pairings
 *
 * You are given two 0-indexed arrays, nums1 and nums2, consisting of non-negative integers. Let there be another array, nums3, which contains the bitwise XOR of all pairings of integers between nums1 and nums2 (every integer in nums1 is paired with every integer in nums2 exactly once).
 * Return the bitwise XOR of all integers in nums3.
 *  
 * Example 1:
 *
 * Input: nums1 = [2,1,3], nums2 = [10,2,5,0]
 * Output: 13
 * Explanation:
 * A possible nums3 array is [8,0,7,2,11,3,4,1,9,1,6,3].
 * The bitwise XOR of all these numbers is 13, so we return 13.
 *
 * Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 0
 * Explanation:
 * All possible pairs of bitwise XORs are nums1[0] ^ nums2[0], nums1[0] ^ nums2[1], nums1[1] ^ nums2[0],
 * and nums1[1] ^ nums2[1].
 * Thus, one possible nums3 array is [2,5,1,6].
 * 2 ^ 5 ^ 1 ^ 6 = 0, so we return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 10^5
 * 	0 <= nums1[i], nums2[j] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-xor-of-all-pairings/
// discuss: https://leetcode.com/problems/bitwise-xor-of-all-pairings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::ops::BitXor;
        nums1
            .iter()
            .take((nums2.len() & 1) * usize::MAX)
            .chain(nums2.iter().take((nums1.len() & 1) * usize::MAX))
            .fold(0, i32::bitxor)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2425_example_1() {
        let nums1 = vec![2, 1, 3];
        let nums2 = vec![10, 2, 5, 0];

        let result = 13;

        assert_eq!(Solution::xor_all_nums(nums1, nums2), result);
    }

    #[test]
    fn test_2425_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        let result = 0;

        assert_eq!(Solution::xor_all_nums(nums1, nums2), result);
    }
}
