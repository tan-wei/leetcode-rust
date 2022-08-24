/**
 * [0801] Minimum Swaps To Make Sequences Increasing
 *
 * You are given two integer arrays of the same length nums1 and nums2. In one operation, you are allowed to swap nums1[i] with nums2[i].
 *
 * 	For example, if nums1 = [1,2,3,<u>8</u>], and nums2 = [5,6,7,<u>4</u>], you can swap the element at i = 3 to obtain nums1 = [1,2,3,4] and nums2 = [5,6,7,8].
 *
 * Return the minimum number of needed operations to make nums1 and nums2 strictly increasing. The test cases are generated so that the given input always makes it possible.
 * An array arr is strictly increasing if and only if arr[0] < arr[1] < arr[2] < ... < arr[arr.length - 1].
 *  
 * Example 1:
 *
 * Input: nums1 = [1,3,5,4], nums2 = [1,2,3,7]
 * Output: 1
 * Explanation:
 * Swap nums1[3] and nums2[3]. Then the sequences are:
 * nums1 = [1, 3, 5, 7] and nums2 = [1, 2, 3, 4]
 * which are both strictly increasing.
 *
 * Example 2:
 *
 * Input: nums1 = [0,3,5,8,9], nums2 = [2,1,4,6,9]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	2 <= nums1.length <= 10^5
 * 	nums2.length == nums1.length
 * 	0 <= nums1[i], nums2[i] <= 2 * 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing/
// discuss: https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut n1 = 0;
        let mut s1 = 1;
        for i in 1..n {
            let mut n2 = i32::MAX;
            let mut s2 = i32::MAX;
            if nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i] {
                n2 = std::cmp::min(n2, n1);
                s2 = std::cmp::min(s2, s1 + 1);
            }
            if nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i] {
                n2 = std::cmp::min(n2, s1);
                s2 = std::cmp::min(s2, n1 + 1);
            }
            n1 = n2;
            s1 = s2;
        }

        std::cmp::min(n1, s1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0801_example_1() {
        let nums1 = vec![1, 3, 5, 4];
        let nums2 = vec![1, 2, 3, 7];
        let result = 1;

        assert_eq!(Solution::min_swap(nums1, nums2), result);
    }

    #[test]
    fn test_0801_example_2() {
        let nums1 = vec![0, 3, 5, 8, 9];
        let nums2 = vec![2, 1, 4, 6, 9];
        let result = 1;

        assert_eq!(Solution::min_swap(nums1, nums2), result);
    }
}
