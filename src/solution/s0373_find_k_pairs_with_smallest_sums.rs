/**
 * [373] Find K Pairs with Smallest Sums
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
 * Define a pair (u, v) which consists of one element from the first array and one element from the second array.
 * Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * Output: [[1,2],[1,4],[1,6]]
 * Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 * Example 2:
 *
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [[1,1],[1,1]]
 * Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 * Example 3:
 *
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [[1,3],[2,3]]
 * Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 10^5
 * 	-10^9 <= nums1[i], nums2[i] <= 10^9
 * 	nums1 and nums2 both are sorted in ascending order.
 * 	1 <= k <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
// discuss: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut k = k;
        let mut result = vec![];
        if k == 0 || nums1.is_empty() || nums2.is_empty() {
            return result;
        }
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..std::cmp::min(k as usize, nums1.len()) {
            heap.push(vec![-(nums1[i] + nums2[0]), nums1[i], nums2[0], 0]);
        }
        while k > 0 && !heap.is_empty() {
            k -= 1;
            let cur = heap.pop().unwrap();
            result.push(vec![cur[1], cur[2]]);
            if cur[3] == nums2.len() as i32 - 1 {
                continue;
            }
            heap.push(vec![
                -cur[1] - nums2[cur[3] as usize + 1],
                cur[1],
                nums2[cur[3] as usize + 1],
                cur[3] + 1,
            ]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0373_example_1() {
        let nums1 = vec![1, 7, 11];
        let nums2 = vec![2, 4, 6];
        let k = 3;
        let result = vec![vec![1, 2], vec![1, 4], vec![1, 6]];

        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), result);
    }

    #[test]
    fn test_0373_example_2() {
        let nums1 = vec![1, 1, 2];
        let nums2 = vec![1, 2, 3];
        let k = 2;
        let result = vec![vec![1, 1], vec![1, 1]];

        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), result);
    }

    #[test]
    fn test_0373_example_3() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3];
        let k = 3;
        let result = vec![vec![1, 3], vec![2, 3]];

        assert_eq!(Solution::k_smallest_pairs(nums1, nums2, k), result);
    }
}
