/**
 * [0870] Advantage Shuffle
 *
 * You are given two integer arrays nums1 and nums2 both of the same length. The advantage of nums1 with respect to nums2 is the number of indices i for which nums1[i] > nums2[i].
 * Return any permutation of nums1 that maximizes its advantage with respect to nums2.
 *  
 * Example 1:
 * Input: nums1 = [2,7,11,15], nums2 = [1,10,4,11]
 * Output: [2,11,7,15]
 * Example 2:
 * Input: nums1 = [12,24,8,32], nums2 = [13,25,32,11]
 * Output: [24,32,8,12]
 *  
 * Constraints:
 *
 * 	1 <= nums1.length <= 10^5
 * 	nums2.length == nums1.length
 * 	0 <= nums1[i], nums2[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/advantage-shuffle/
// discuss: https://leetcode.com/problems/advantage-shuffle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        nums1.sort_unstable();
        nums2
            .into_iter()
            .map(|x| {
                let i = nums1.binary_search(&(x + 1)).unwrap_or_else(|i| i);
                nums1.remove(if i < nums1.len() { i } else { 0 })
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0870_example_1() {
        let nums1 = vec![2, 7, 11, 15];
        let nums2 = vec![1, 10, 4, 11];
        let result = vec![2, 11, 7, 15];

        assert_eq!(Solution::advantage_count(nums1, nums2), result);
    }

    #[test]
    fn test_0870_example_2() {
        let nums1 = vec![12, 24, 8, 32];
        let nums2 = vec![13, 25, 32, 11];
        let result = vec![24, 32, 8, 12];

        assert_eq!(Solution::advantage_count(nums1, nums2), result);
    }
}
