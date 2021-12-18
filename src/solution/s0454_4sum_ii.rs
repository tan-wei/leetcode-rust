/**
 * [0454] 4Sum II
 *
 * Given four integer arrays nums1, nums2, nums3, and nums4 all of length n, return the number of tuples (i, j, k, l) such that:
 *
 * 	0 <= i, j, k, l < n
 * 	nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
 *
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
 * Output: 2
 * Explanation:
 * The two tuples are:
 * 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
 * 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
 *
 * Example 2:
 *
 * Input: nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == nums1.length
 * 	n == nums2.length
 * 	n == nums3.length
 * 	n == nums4.length
 * 	1 <= n <= 200
 * 	-2^28 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2^28
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum-ii/
// discuss: https://leetcode.com/problems/4sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut mp = std::collections::HashMap::new();

        for n1 in nums1.iter() {
            for n2 in nums2.iter() {
                *mp.entry(n1 + n2).or_insert(0) += 1;
            }
        }

        let mut count = 0;
        for n3 in nums3.iter() {
            for n4 in nums4.iter() {
                let key = 0 - n3 - n4;
                if let Some(&value) = mp.get(&key) {
                    count += value;
                }
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0454_example_1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        let result = 2;

        assert_eq!(Solution::four_sum_count(nums1, nums2, nums3, nums4), result);
    }
}
