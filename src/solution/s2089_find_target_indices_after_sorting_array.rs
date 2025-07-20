/**
 * [2089] Find Target Indices After Sorting Array
 *
 * You are given a 0-indexed integer array nums and a target element target.
 * A target index is an index i such that nums[i] == target.
 * Return a list of the target indices of nums after sorting nums in non-decreasing order. If there are no target indices, return an empty list. The returned list must be sorted in increasing order.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,5,2,3], target = 2
 * Output: [1,2]
 * Explanation: After sorting, nums is [1,<u>2</u>,<u>2</u>,3,5].
 * The indices where nums[i] == 2 are 1 and 2.
 *
 * Example 2:
 *
 * Input: nums = [1,2,5,2,3], target = 3
 * Output: [3]
 * Explanation: After sorting, nums is [1,2,2,<u>3</u>,5].
 * The index where nums[i] == 3 is 3.
 *
 * Example 3:
 *
 * Input: nums = [1,2,5,2,3], target = 5
 * Output: [4]
 * Explanation: After sorting, nums is [1,2,2,3,<u>5</u>].
 * The index where nums[i] == 5 is 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i], target <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-target-indices-after-sorting-array/
// discuss: https://leetcode.com/problems/find-target-indices-after-sorting-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        let from = nums.partition_point(|&x| x < target) as i32;
        let to = nums.partition_point(|&x| x <= target) as i32;

        (from..to).collect::<Vec<i32>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2089_example_1() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 2;

        let result = vec![1, 2];

        assert_eq!(Solution::target_indices(nums, target), result);
    }

    #[test]
    fn test_2089_example_2() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 3;

        let result = vec![3];

        assert_eq!(Solution::target_indices(nums, target), result);
    }

    #[test]
    fn test_2089_example_3() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 5;

        let result = vec![4];

        assert_eq!(Solution::target_indices(nums, target), result);
    }
}
