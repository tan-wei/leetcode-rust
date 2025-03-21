/**
 * [1929] Concatenation of Array
 *
 * Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
 * Specifically, ans is the concatenation of two nums arrays.
 * Return the array ans.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1]
 * Output: [1,2,1,1,2,1]
 * Explanation: The array ans is formed as follows:
 * - ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
 * - ans = [1,2,1,1,2,1]
 * Example 2:
 *
 * Input: nums = [1,3,2,1]
 * Output: [1,3,2,1,1,3,2,1]
 * Explanation: The array ans is formed as follows:
 * - ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
 * - ans = [1,3,2,1,1,3,2,1]
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 1000
 * 	1 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenation-of-array/
// discuss: https://leetcode.com/problems/concatenation-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        let len = result.len();

        result.reserve_exact(len);

        for i in 0..len {
            result.push(result[i]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1929_example_1() {
        let nums = vec![1, 2, 1];
        let result = vec![1, 2, 1, 1, 2, 1];

        assert_eq!(Solution::get_concatenation(nums), result);
    }

    #[test]
    fn test_1929_example_2() {
        let nums = vec![1, 2, 1];
        let result = vec![1, 2, 1, 1, 2, 1];

        assert_eq!(Solution::get_concatenation(nums), result);
    }
}
