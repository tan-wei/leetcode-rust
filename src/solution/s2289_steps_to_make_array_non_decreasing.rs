/**
 * [2289] Steps to Make Array Non-decreasing
 *
 * You are given a 0-indexed integer array nums. In one step, remove all elements nums[i] where nums[i - 1] > nums[i] for all 0 < i < nums.length.
 * Return the number of steps performed until nums becomes a non-decreasing array.
 *  
 * Example 1:
 *
 * Input: nums = [5,3,4,4,7,3,6,11,8,5,11]
 * Output: 3
 * Explanation: The following are the steps performed:
 * - Step 1: [5,<u>3</u>,4,4,7,<u>3</u>,6,11,<u>8</u>,<u>5</u>,11] becomes [5,4,4,7,6,11,11]
 * - Step 2: [5,<u>4</u>,4,7,<u>6</u>,11,11] becomes [5,4,7,11,11]
 * - Step 3: [5,<u>4</u>,7,11,11] becomes [5,7,11,11]
 * [5,7,11,11] is a non-decreasing array. Therefore, we return 3.
 *
 * Example 2:
 *
 * Input: nums = [4,5,7,7,13]
 * Output: 0
 * Explanation: nums is already a non-decreasing array. Therefore, we return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/steps-to-make-array-non-decreasing/
// discuss: https://leetcode.com/problems/steps-to-make-array-non-decreasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = vec![(nums[0], 0)];
        let mut result = 0;

        for &v in &nums[1..] {
            let mut count = 0;
            while !stack.is_empty() && stack[stack.len() - 1].0 <= v {
                let current = stack.pop().unwrap_or((0, 0)).1;
                count = std::cmp::max(count, current);
            }

            if !stack.is_empty() {
                count += 1;
            }

            stack.push((v, count));
            result = std::cmp::max(count, result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2289_example_1() {
        let nums = vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11];

        let result = 3;

        assert_eq!(Solution::total_steps(nums), result);
    }

    #[test]
    fn test_2289_example_2() {
        let nums = vec![4, 5, 7, 7, 13];

        let result = 0;

        assert_eq!(Solution::total_steps(nums), result);
    }
}
