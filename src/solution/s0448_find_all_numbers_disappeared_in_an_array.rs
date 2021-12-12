/**
 * [0448] Find All Numbers Disappeared in an Array
 *
 * Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
 *  
 * Example 1:
 * Input: nums = [4,3,2,7,8,2,3,1]
 * Output: [5,6]
 * Example 2:
 * Input: nums = [1,1]
 * Output: [2]
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i] <= n
 *
 *  
 * Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// discuss: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let m = (nums[i]).abs() as usize - 1; // index start from 0
            nums[m] = if nums[m] > 0 { -nums[m] } else { nums[m] };
        }
        let mut result = vec![];
        for i in 0..nums.len() {
            if nums[i] > 0 {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0448_example_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let result = vec![5, 6];

        assert_eq_sorted!(Solution::find_disappeared_numbers(nums), result);
    }

    #[test]
    fn test_0448_example_2() {
        let nums = vec![1, 1];
        let result = vec![2];

        assert_eq_sorted!(Solution::find_disappeared_numbers(nums), result);
    }
}
