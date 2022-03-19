/**
 * [0581] Shortest Unsorted Continuous Subarray
 *
 * Given an integer array nums, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order.
 * Return the shortest such subarray and output its length.
 *  
 * Example 1:
 *
 * Input: nums = [2,6,4,8,10,9,15]
 * Output: 5
 * Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: 0
 *
 * Example 3:
 *
 * Input: nums = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^5 <= nums[i] <= 10^5
 *
 *  
 * Follow up: Can you solve it in O(n) time complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/
// discuss: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/shortest-unsorted-continuous-subarray/discuss/1188866/Rust-or-Two-pass-or-Time-O(n)-or-Space-O(1)
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max = nums[end];
        for i in 1..nums.len() {
            if nums[i] < max {
                end = i + 1;
            } else {
                max = nums[i];
            }
        }

        if end == 0 {
            return 0;
        }

        let mut start = nums.len() - 1;
        let mut min = nums[start];
        for i in (0..start).rev() {
            if nums[i] > min {
                start = i;
            } else {
                min = nums[i];
            }
        }

        (end - start) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0581_example_1() {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        let result = 5;

        assert_eq!(Solution::find_unsorted_subarray(nums), result);
    }

    #[test]
    fn test_0581_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = 0;

        assert_eq!(Solution::find_unsorted_subarray(nums), result);
    }

    #[test]
    fn test_0581_example_3() {
        let nums = vec![1];
        let result = 0;

        assert_eq!(Solution::find_unsorted_subarray(nums), result);
    }
}
