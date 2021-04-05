/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 * The number of elements initialized in nums1 and nums2 are m and n respectively. You may assume that nums1 has a size equal to m + n such that it has enough space to hold additional elements from nums2.
 *  
 * Example 1:
 * Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * Output: [1,2,2,3,5,6]
 * Example 2:
 * Input: nums1 = [1], m = 1, nums2 = [], n = 0
 * Output: [1]
 *  
 * Constraints:
 *
 * 	nums1.length == m + n
 * 	nums2.length == n
 * 	0 <= m, n <= 200
 * 	1 <= m + n <= 200
 * 	-10^9 <= nums1[i], nums2[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-sorted-array/
// discuss: https://leetcode.com/problems/merge-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut cur_1 = m - 1;
        let mut cur_2 = n - 1;
        let mut i = m + n - 1;

        while cur_1 >= 0 && cur_2 >= 0 {
            if nums1[cur_1 as usize] > nums2[cur_2 as usize] {
                nums1[i as usize] = nums1[cur_1 as usize];
                cur_1 -= 1;
            } else {
                nums1[i as usize] = nums2[cur_2 as usize];
                cur_2 -= 1;
            }
            i -= 1;
        }

        while cur_1 >= 0 {
            nums1[i as usize] = nums1[cur_1 as usize];
            cur_1 -= 1;
            i -= 1;
        }

        while cur_2 >= 0 {
            nums1[i as usize] = nums2[cur_2 as usize];
            cur_2 -= 1;
            i -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0088_example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let result = vec![1, 2, 2, 3, 5, 6];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, result);
    }

    #[test]
    fn test_0088_example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let result = vec![1];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, result);
    }
}
