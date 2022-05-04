/**
 * [0665] Non-decreasing Array
 *
 * Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most one element.
 * We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).
 *  
 * Example 1:
 *
 * Input: nums = [4,2,3]
 * Output: true
 * Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
 *
 * Example 2:
 *
 * Input: nums = [4,2,1]
 * Output: false
 * Explanation: You can't get a non-decreasing array by modify at most one element.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^4
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/non-decreasing-array/
// discuss: https://leetcode.com/problems/non-decreasing-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut nums = nums;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                count += 1;
                if (i < 2 || nums[i - 2] <= nums[i]) {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
                if count > 1 {
                    break;
                }
            }
        }
        count <= 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0665_example_1() {
        let nums = vec![4, 2, 3];
        let result = true;

        assert_eq!(Solution::check_possibility(nums), result);
    }

    #[test]
    fn test_0665_example_2() {
        let nums = vec![4, 2, 1];
        let result = false;

        assert_eq!(Solution::check_possibility(nums), result);
    }
}
