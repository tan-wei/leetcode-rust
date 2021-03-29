/**
 * [81] Search in Rotated Sorted Array II
 *
 * There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).
 * Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].
 * Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.
 *  
 * Example 1:
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * Example 2:
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is guaranteed to be rotated at some pivot.
 * 	-10^4 <= target <= 10^4
 *
 *  
 * Follow up: This problem is the same as <a href="/problems/search-in-rotated-sorted-array/description/" target="_blank">Search in Rotated Sorted Array</a>, where nums may contain duplicates. Would this affect the runtime complexity? How and why?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let (mut l, mut mid, mut h) = (0, 0, nums.len() - 1);

        while l < h {
            mid = (h + l) >> 1;
            if (nums[mid] == target) {
                return true;
            } else if (nums[mid] > nums[h]) {
                if nums[mid] > target && nums[l] <= target {
                    h = mid;
                } else {
                    l = mid + 1;
                }
            } else if nums[mid] < nums[h] {
                if nums[mid] < target && nums[h] >= target {
                    l = mid + 1;
                } else {
                    h = mid;
                }
            } else {
                h -= 1;
            }
        }

        nums[l] == target
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0081_example_1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let result = true;

        assert_eq!(Solution::search(nums, target), result);
    }

    #[test]
    fn test_0081_example_2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        let result = false;

        assert_eq!(Solution::search(nums, target), result);
    }
}
