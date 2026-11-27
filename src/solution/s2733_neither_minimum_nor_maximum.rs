/**
 * [2733] Neither Minimum nor Maximum
 *
 * Given an integer array nums containing distinct positive integers, find and return any number from the array that is neither the minimum nor the maximum value in the array, or -1 if there is no such number.
 * Return the selected integer.
 *  
 * Example 1:
 *
 * Input: nums = [3,2,1,4]
 * Output: 2
 * Explanation: In this example, the minimum value is 1 and the maximum value is 4. Therefore, either 2 or 3 can be valid answers.
 *
 * Example 2:
 *
 * Input: nums = [1,2]
 * Output: -1
 * Explanation: Since there is no number in nums that is neither the maximum nor the minimum, we cannot select a number that satisfies the given condition. Therefore, there is no answer.
 *
 * Example 3:
 *
 * Input: nums = [2,1,3]
 * Output: 2
 * Explanation: Since 2 is neither the maximum nor the minimum value in nums, it is the only valid answer.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i] <= 100
 * 	All values in nums are distinct
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/neither-minimum-nor-maximum/
// discuss: https://leetcode.com/problems/neither-minimum-nor-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            -1
        } else {
            let mut nums = nums;
            let mut slc = &mut nums[..3];
            if slc[0] > slc[1] {
                slc.swap(0, 1);
            }
            if slc[1] > slc[2] {
                slc.swap(1, 2);
            }
            if slc[0] > slc[1] {
                slc.swap(0, 1);
            }
            slc[1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2733_example_1() {
        let nums = vec![3, 2, 1, 4];

        let result = 2;

        assert_eq!(Solution::find_non_min_or_max(nums), result);
    }

    #[test]
    fn test_2733_example_2() {
        let nums = vec![1, 2];

        let result = -1;

        assert_eq!(Solution::find_non_min_or_max(nums), result);
    }

    #[test]
    fn test_2733_example_3() {
        let nums = vec![2, 1, 3];

        let result = 2;

        assert_eq!(Solution::find_non_min_or_max(nums), result);
    }
}
