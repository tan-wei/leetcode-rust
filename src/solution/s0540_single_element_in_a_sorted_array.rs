/**
 * [0540] Single Element in a Sorted Array
 *
 * You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once.
 * Return the single element that appears only once.
 * Your solution must run in O(log n) time and O(1) space.
 *  
 * Example 1:
 * Input: nums = [1,1,2,3,3,4,4,8,8]
 * Output: 2
 * Example 2:
 * Input: nums = [3,3,7,7,10,11,11]
 * Output: 10
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-element-in-a-sorted-array/
// discuss: https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if (mid % 2 == 0) == (nums[mid] != nums[mid + 1]) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        nums[lo]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0540_example_1() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        let result = 2;

        assert_eq!(Solution::single_non_duplicate(nums), result);
    }

    #[test]
    fn test_0540_example_2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        let result = 10;

        assert_eq!(Solution::single_non_duplicate(nums), result);
    }
}
