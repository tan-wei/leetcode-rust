/**
 * [350] Intersection of Two Arrays II
 *
 * Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
 *  
 * Example 1:
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2,2]
 *
 * Example 2:
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [4,9]
 * Explanation: [9,4] is also accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 1000
 *
 *  
 * Follow up:
 *
 * 	What if the given array is already sorted? How would you optimize your algorithm?
 * 	What if nums1's size is small compared to nums2's size? Which algorithm is better?
 * 	What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-two-arrays-ii/
// discuss: https://leetcode.com/problems/intersection-of-two-arrays-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut m = nums1
            .iter()
            .fold(std::collections::HashMap::new(), |mut m, e| {
                m.entry(e).and_modify(|v| *v += 1).or_insert(1);
                m
            });
        nums2.iter().fold(vec![], |mut ret, e| {
            m.entry(e).and_modify(|e2| {
                *e2 -= 1;
                if *e2 >= 0 {
                    ret.push(*e)
                }
            });
            ret
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0350_example_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = vec![2, 2];

        assert_eq_sorted!(Solution::intersect(nums1, nums2), result);
    }

    #[test]
    fn test_0350_example_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = vec![4, 9];

        assert_eq_sorted!(Solution::intersect(nums1, nums2), result);
    }
}
