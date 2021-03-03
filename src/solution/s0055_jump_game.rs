/**
 * [55] Jump Game
 *
 * Given an array of non-negative integers nums, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Determine if you are able to reach the last index.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Example 2:
 *
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game/
// discuss: https://leetcode.com/problems/jump-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for (i, val) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            let val = *val as usize;
            max_reach = std::cmp::max(max_reach, i + val);
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0055_example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = true;

        assert_eq!(Solution::can_jump(nums), result);
    }

    #[test]
    fn test_0055_example_2() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = false;

        assert_eq!(Solution::can_jump(nums), result);
    }
}
