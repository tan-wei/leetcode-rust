/**
 * [2179] Count Good Triplets in an Array
 *
 * You are given two 0-indexed arrays nums1 and nums2 of length n, both of which are permutations of [0, 1, ..., n - 1].
 * A good triplet is a set of 3 distinct values which are present in increasing order by position both in nums1 and nums2. In other words, if we consider pos1v as the index of the value v in nums1 and pos2v as the index of the value v in nums2, then a good triplet will be a set (x, y, z) where 0 <= x, y, z <= n - 1, such that pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.
 * Return the total number of good triplets.
 *  
 * Example 1:
 *
 * Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
 * Output: 1
 * Explanation:
 * There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3).
 * Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.
 *
 * Example 2:
 *
 * Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
 * Output: 4
 * Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).
 *
 *  
 * Constraints:
 *
 * 	n == nums1.length == nums2.length
 * 	3 <= n <= 10^5
 * 	0 <= nums1[i], nums2[i] <= n - 1
 * 	nums1 and nums2 are permutations of [0, 1, ..., n - 1].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-good-triplets-in-an-array/
// discuss: https://leetcode.com/problems/count-good-triplets-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2179_example_1() {
        let nums1 = vec![2, 0, 1, 3];
        let nums2 = vec![0, 1, 2, 3];

        let result = 1;

        assert_eq!(Solution::good_triplets(nums1, nums2), result);
    }

    #[test]
    #[ignore]
    fn test_2179_example_2() {
        let nums1 = vec![4, 0, 1, 3, 2];
        let nums2 = vec![4, 1, 0, 2, 3];

        let result = 4;

        assert_eq!(Solution::good_triplets(nums1, nums2), result);
    }
}
