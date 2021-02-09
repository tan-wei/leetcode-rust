/**
 * [33] Search in Rotated Sorted Array
 *
 * You are given an integer array nums sorted in ascending order (with distinct values), and an integer target.
 * Suppose that nums is rotated at some pivot unknown to you beforehand (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 * If target is found in the array return its index, otherwise, return -1.
 *  
 * Example 1:
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * Example 2:
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * Example 3:
 * Input: nums = [1], target = 0
 * Output: -1
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	All values of nums are unique.
 * 	nums is guaranteed to be rotated at some pivot.
 * 	-10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/223659/Rust-Concise-O(log-N)-Binary-Search-Solution-without-Founding-Pivot-0-ms-beats-100
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        if size == 0 {
            return -1;
        }

        let mut base = 0_usize;

        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if nums[mid] == target {
                return mid as i32;
            }
            // we split the ring to [base..half] & [half+1..base-1]
            // if target not in [base..half] ring, move base to select another ring
            if !(((nums[base] < nums[mid]) && (target >= nums[base] && target <= nums[mid]))
                || ((nums[base] > nums[mid]) && (target >= nums[base] || target <= nums[mid])))
            {
                base = mid;
            }
            size -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0033_example_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;

        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_0033_example_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;

        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_0033_example_3() {
        let nums = vec![1];
        let target = 0;

        assert_eq!(Solution::search(nums, target), -1);
    }
}
