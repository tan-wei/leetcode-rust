/**
 * [349] Intersection of Two Arrays
 *
 * Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2]
 *
 * Example 2:
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [9,4]
 * Explanation: [4,9] is also accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-two-arrays/
// discuss: https://leetcode.com/problems/intersection-of-two-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set_1: std::collections::HashSet<_> = nums1.iter().cloned().collect();
        let set_2: std::collections::HashSet<_> = nums2.iter().cloned().collect();
        let mut result = Vec::new();
        for &x in set_1.intersection(&set_2) {
            result.push(x);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0349_example_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = vec![2];

        assert_eq_sorted!(Solution::intersection(nums1, nums2), result);
    }

    #[test]
    fn test_0349_example_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = vec![4, 9];

        assert_eq_sorted!(Solution::intersection(nums1, nums2), result);
    }
}
