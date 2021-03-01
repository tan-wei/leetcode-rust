/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *  
 * Example 1:
 *
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 * Example 2:
 *
 * Input: nums = [1]
 * Output: 1
 *
 * Example 3:
 *
 * Input: nums = [0]
 * Output: 0
 *
 * Example 4:
 *
 * Input: nums = [-1]
 * Output: -1
 *
 * Example 5:
 *
 * Input: nums = [-100000]
 * Output: -100000
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^5 <= nums[i] <= 10^5
 *
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray/
// discuss: https://leetcode.com/problems/maximum-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, i32::MIN), |(cur, mx), &num| {
                (std::cmp::max(0, cur + num), std::cmp::max(mx, cur + num))
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0053_example_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = 6;

        assert_eq!(Solution::max_sub_array(nums), result);
    }

    #[test]
    fn test_0053_example_2() {
        let nums = vec![1];
        let result = 1;

        assert_eq!(Solution::max_sub_array(nums), result);
    }

    #[test]
    fn test_0053_example_3() {
        let nums = vec![0];
        let result = 0;

        assert_eq!(Solution::max_sub_array(nums), result);
    }

    #[test]
    fn test_0053_example_4() {
        let nums = vec![-1];
        let result = -1;

        assert_eq!(Solution::max_sub_array(nums), result);
    }

    #[test]
    fn test_0053_example_5() {
        let nums = vec![-100000];
        let result = -100000;

        assert_eq!(Solution::max_sub_array(nums), result);
    }
}
