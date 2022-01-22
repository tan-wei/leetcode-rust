/**
 * [0496] Next Greater Element I
 *
 * The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
 * You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
 * For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
 * Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.
 *  
 * Example 1:
 *
 * Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
 * Output: [-1,3,-1]
 * Explanation: The next greater element for each value of nums1 is as follows:
 * - 4 is underlined in nums2 = [1,3,<u>4</u>,2]. There is no next greater element, so the answer is -1.
 * - 1 is underlined in nums2 = [<u>1</u>,3,4,2]. The next greater element is 3.
 * - 2 is underlined in nums2 = [1,3,4,<u>2</u>]. There is no next greater element, so the answer is -1.
 *
 * Example 2:
 *
 * Input: nums1 = [2,4], nums2 = [1,2,3,4]
 * Output: [3,-1]
 * Explanation: The next greater element for each value of nums1 is as follows:
 * - 2 is underlined in nums2 = [1,<u>2</u>,3,4]. The next greater element is 3.
 * - 4 is underlined in nums2 = [1,2,3,<u>4</u>]. There is no next greater element, so the answer is -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length <= nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 10^4
 * 	All integers in nums1 and nums2 are unique.
 * 	All the integers of nums1 also appear in nums2.
 *
 *  
 * Follow up: Could you find an O(nums1.length + nums2.length) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-i/
// discuss: https://leetcode.com/problems/next-greater-element-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums1.len() {
            let mut j = 0;
            while j < nums2.len() {
                if nums1[i] == nums2[j] {
                    break;
                }
                j += 1;
            }

            while j < nums2.len() {
                if nums1[i] < nums2[j] {
                    result.push(nums2[j]);
                    break;
                }
                j += 1;
            }

            if j == nums2.len() {
                result.push(-1);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0496_example_1() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let result = vec![-1, 3, -1];

        assert_eq!(Solution::next_greater_element(nums1, nums2), result);
    }

    #[test]
    fn test_0496_example_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let result = vec![3, -1];

        assert_eq!(Solution::next_greater_element(nums1, nums2), result);
    }
}
