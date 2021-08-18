/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 * Note that you must do this in-place without making a copy of the array.
 *  
 * Example 1:
 * Input: nums = [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 * Example 2:
 * Input: nums = [0]
 * Output: [0]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you minimize the total number of operations done?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/move-zeroes/
// discuss: https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&number| number != 0);
        while nums.len() < len {
            nums.push(0);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0283_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let result = vec![1, 3, 12, 0, 0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, result);
    }

    #[test]
    fn test_0283_example_2() {
        let mut nums = vec![0];
        let result = vec![0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, result);
    }
}
