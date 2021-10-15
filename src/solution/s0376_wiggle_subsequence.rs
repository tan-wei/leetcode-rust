/**
 * [376] Wiggle Subsequence
 *
 * A wiggle sequence is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.
 *
 * 	For example, [1, 7, 4, 9, 2, 5] is a wiggle sequence because the differences (6, -3, 5, -7, 3) alternate between positive and negative.
 * 	In contrast, [1, 4, 7, 2, 5] and [1, 7, 4, 5, 5] are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.
 *
 * A subsequence is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.
 * Given an integer array nums, return the length of the longest wiggle subsequence of nums.
 *  
 * Example 1:
 *
 * Input: nums = [1,7,4,9,2,5]
 * Output: 6
 * Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
 *
 * Example 2:
 *
 * Input: nums = [1,17,5,10,13,15,10,5,16,8]
 * Output: 7
 * Explanation: There are several subsequences that achieve this length.
 * One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4,5,6,7,8,9]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 1000
 *
 *  
 * Follow up: Could you solve this in O(n) time?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wiggle-subsequence/
// discuss: https://leetcode.com/problems/wiggle-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        if (size == 0) {
            return 0;
        }
        let mut up = vec![0; size];
        let mut down = vec![0; size];
        up[0] = 1;
        down[0] = 1;
        for i in 1..size {
            if nums[i] > nums[i - 1] {
                up[i] = down[i - 1] + 1;
                down[i] = down[i - 1];
            } else if nums[i] < nums[i - 1] {
                down[i] = up[i - 1] + 1;
                up[i] = up[i - 1];
            } else {
                up[i] = up[i - 1];
                down[i] = down[i - 1];
            }
        }

        std::cmp::max(up[size - 1], down[size - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0376_example_1() {
        let nums = vec![1, 7, 4, 9, 2, 5];
        let result = 6;

        assert_eq!(Solution::wiggle_max_length(nums), result);
    }

    #[test]
    fn test_0376_example_2() {
        let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        let result = 7;

        assert_eq!(Solution::wiggle_max_length(nums), result);
    }

    #[test]
    fn test_0376_example_3() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = 2;

        assert_eq!(Solution::wiggle_max_length(nums), result);
    }
}
