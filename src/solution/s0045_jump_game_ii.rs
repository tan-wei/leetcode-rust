/**
 * [45] Jump Game II
 *
 * Given an array of non-negative integers nums, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Your goal is to reach the last index in the minimum number of jumps.
 * You can assume that you can always reach the last index.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Example 2:
 *
 * Input: nums = [2,3,0,1,4]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-ii/
// discuss: https://leetcode.com/problems/jump-game-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut prev_line = 0;
        let mut swapping_line = 0;
        let mut time = 0;
        loop {
            if swapping_line + 1 >= nums.len() {
                return time;
            }

            let mut mx = 0;
            while prev_line <= swapping_line {
                mx = mx.max(nums[prev_line] as usize + prev_line);
                prev_line += 1;
            }
            swapping_line = mx;
            time += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0045_exmample_1() {
        let nums = vec![2, 3, 1, 1, 4];

        assert_eq!(Solution::jump(nums), 2);
    }

    #[test]
    fn test_0045_exmample_2() {
        let nums = vec![2, 3, 0, 1, 4];

        assert_eq!(Solution::jump(nums), 2);
    }
}
