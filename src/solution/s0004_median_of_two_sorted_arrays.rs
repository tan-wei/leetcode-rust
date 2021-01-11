/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * Follow up: The overall run time complexity should be O(log (m+n)).
 *  
 * Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 * Example 3:
 *
 * Input: nums1 = [0,0], nums2 = [0,0]
 * Output: 0.00000
 *
 * Example 4:
 *
 * Input: nums1 = [], nums2 = [1]
 * Output: 1.00000
 *
 * Example 5:
 *
 * Input: nums1 = [2], nums2 = []
 * Output: 2.00000
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = vec![];
        let (mut i, mut j) = (0, 0);

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }

        while i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        }

        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }

        let mid = merged.len() / 2;
        match merged.len() % 2 {
            0 => return ((merged[mid - 1] as f64 + merged[mid] as f64) / 2.0) as f64,
            _ => return merged[mid] as f64,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0004_example_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test_0004_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_0004_example_3() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 0.0);
    }

    #[test]
    fn test_0004_example_4() {
        let nums1 = vec![];
        let nums2 = vec![1];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test_0004_example_5() {
        let nums1 = vec![2];
        let nums2 = vec![];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }
}
