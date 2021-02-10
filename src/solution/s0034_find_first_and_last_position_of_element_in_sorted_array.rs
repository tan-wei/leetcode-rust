/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * Follow up: Could you write an algorithm with O(log n) runtime complexity?
 *  
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums is a non-decreasing array.
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::Ordering;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower = nums
            .binary_search_by(|&val| match val.cmp(&target) {
                Ordering::Equal => Ordering::Greater,
                result => result,
            })
            .unwrap_err();

        let upper = nums
            .binary_search_by(|&val| match val.cmp(&target) {
                Ordering::Equal => Ordering::Less,
                result => result,
            })
            .unwrap_err();

        if lower == upper {
            return vec![-1, -1];
        }

        vec![lower as i32, (upper - 1) as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0034_example_1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;

        assert_eq!(Solution::search_range(nums, target), vec![3, 4]);
    }

    #[test]
    fn test_0034_example_2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;

        assert_eq!(Solution::search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn test_0034_example_3() {
        let nums = vec![];
        let target = 0;

        assert_eq!(Solution::search_range(nums, target), vec![-1, -1]);
    }
}
