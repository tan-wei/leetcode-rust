/**
 * [0962] Maximum Width Ramp
 *
 * A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j]. The width of such a ramp is j - i.
 * Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.
 *  
 * Example 1:
 *
 * Input: nums = [6,0,8,2,1,5]
 * Output: 4
 * Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] = 0 and nums[5] = 5.
 *
 * Example 2:
 *
 * Input: nums = [9,8,1,0,1,9,4,0,4,1]
 * Output: 7
 * Explanation: The maximum width ramp is achieved at (i, j) = (2, 9): nums[2] = 1 and nums[9] = 1.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 5 * 10^4
 * 	0 <= nums[i] <= 5 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-width-ramp/
// discuss: https://leetcode.com/problems/maximum-width-ramp/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut idx = (0..len as i32).collect::<Vec<i32>>();
        idx.sort_by(|&i, &j| nums[i as usize].cmp(&nums[j as usize]));

        let mut result = 0;
        let mut min_i = len as i32;

        for i in idx {
            result = result.max(i - min_i);
            min_i = min_i.min(i);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0962_example_1() {
        let nums = vec![6, 0, 8, 2, 1, 5];
        let result = 4;

        assert_eq!(Solution::max_width_ramp(nums), result);
    }

    #[test]
    fn test_0962_example_2() {
        let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
        let result = 7;

        assert_eq!(Solution::max_width_ramp(nums), result);
    }
}
