/**
 * [321] Create Maximum Number
 *
 * You are given two integer arrays nums1 and nums2 of lengths m and n respectively. nums1 and nums2 represent the digits of two numbers. You are also given an integer k.
 * Create the maximum number of length k <= m + n from digits of the two numbers. The relative order of the digits from the same array must be preserved.
 * Return an array of the k digits representing the answer.
 *  
 * Example 1:
 *
 * Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
 * Output: [9,8,6,5,3]
 *
 * Example 2:
 *
 * Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
 * Output: [6,7,6,0,4]
 *
 * Example 3:
 *
 * Input: nums1 = [3,9], nums2 = [8,9], k = 3
 * Output: [9,8,9]
 *
 *  
 * Constraints:
 *
 * 	m == nums1.length
 * 	n == nums2.length
 * 	1 <= m, n <= 500
 * 	0 <= nums1[i], nums2[i] <= 9
 * 	1 <= k <= m + n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/create-maximum-number/
// discuss: https://leetcode.com/problems/create-maximum-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let mut max_merged = None;
        for size1 in 0..=k.min(n1) {
            let size2 = k - size1;
            if size2 > n2 {
                continue;
            }
            let max1 = Self::max_one(&nums1, size1);
            let max2 = Self::max_one(&nums2, size2);
            let max3 = Self::max_merge(max1, max2);
            if let Some(max) = max_merged {
                if max < max3 {
                    max_merged = Some(max3);
                } else {
                    max_merged = Some(max);
                }
            } else {
                max_merged = Some(max3);
            }
        }
        max_merged.unwrap()
    }

    fn max_one(nums: &[i32], k: usize) -> Vec<i32> {
        let mut stack = Vec::new();
        let n = nums.len();
        for (i, &num) in nums.iter().enumerate().take(n) {
            let right = n - i;
            while let Some(&top) = stack.last() {
                if top < num && stack.len() + right > k {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        while stack.len() > k {
            stack.pop();
        }
        stack
    }

    fn max_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;
        loop {
            if i < nums1.len() && j < nums2.len() {
                if Self::greater(&nums1, &nums2, i, j) {
                    res.push(nums1[i]);
                    i += 1;
                } else {
                    res.push(nums2[j]);
                    j += 1;
                }
                continue;
            }
            if i < nums1.len() {
                res.push(nums1[i]);
                i += 1;
                continue;
            }
            if j < nums2.len() {
                res.push(nums2[j]);
                j += 1;
                continue;
            }
            break;
        }
        res
    }

    fn greater(nums1: &[i32], nums2: &[i32], mut i: usize, mut j: usize) -> bool {
        while i < nums1.len() && j < nums2.len() && nums1[i] == nums2[j] {
            i += 1;
            j += 1;
        }
        j == nums2.len() || (i < nums1.len() && nums1[i] > nums2[j])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0321_example_1() {
        let nums1 = vec![3, 4, 6, 5];
        let nums2 = vec![9, 1, 2, 5, 8, 3];
        let k = 5;

        let result = vec![9, 8, 6, 5, 3];

        assert_eq!(Solution::max_number(nums1, nums2, k), result);
    }

    #[test]
    fn test_0321_example_2() {
        let nums1 = vec![6, 7];
        let nums2 = vec![6, 0, 4];
        let k = 5;

        let result = vec![6, 7, 6, 0, 4];

        assert_eq!(Solution::max_number(nums1, nums2, k), result);
    }

    #[test]
    fn test_0321_example_3() {
        let nums1 = vec![3, 9];
        let nums2 = vec![8, 9];
        let k = 3;

        let result = vec![9, 8, 9];

        assert_eq!(Solution::max_number(nums1, nums2, k), result);
    }
}
