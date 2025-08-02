/**
 * [2104] Sum of Subarray Ranges
 *
 * You are given an integer array nums. The range of a subarray of nums is the difference between the largest and smallest element in the subarray.
 * Return the sum of all subarray ranges of nums.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: 4
 * Explanation: The 6 subarrays of nums are the following:
 * [1], range = largest - smallest = 1 - 1 = 0
 * [2], range = 2 - 2 = 0
 * [3], range = 3 - 3 = 0
 * [1,2], range = 2 - 1 = 1
 * [2,3], range = 3 - 2 = 1
 * [1,2,3], range = 3 - 1 = 2
 * So the sum of all ranges is 0 + 0 + 0 + 1 + 1 + 2 = 4.
 * Example 2:
 *
 * Input: nums = [1,3,3]
 * Output: 4
 * Explanation: The 6 subarrays of nums are the following:
 * [1], range = largest - smallest = 1 - 1 = 0
 * [3], range = 3 - 3 = 0
 * [3], range = 3 - 3 = 0
 * [1,3], range = 3 - 1 = 2
 * [3,3], range = 3 - 3 = 0
 * [1,3,3], range = 3 - 1 = 2
 * So the sum of all ranges is 0 + 0 + 0 + 2 + 0 + 2 = 4.
 *
 * Example 3:
 *
 * Input: nums = [4,-2,-3,4,1]
 * Output: 59
 * Explanation: The sum of all subarray ranges of nums is 59.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	-10^9 <= nums[i] <= 10^9
 *
 *  
 * Follow-up: Could you find a solution with O(n) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subarray-ranges/
// discuss: https://leetcode.com/problems/sum-of-subarray-ranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-subarray-ranges/solutions/2502690/rust-solution/
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;

        for i in 0..nums.len() {
            let (mut min, mut max) = (nums[i], nums[i]);

            for j in i..nums.len() {
                min = min.min(nums[j]);
                max = max.max(nums[j]);

                result += (max - min) as i64;
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
    fn test_2104_example_1() {
        let nums = vec![1, 2, 3];

        let result = 4;

        assert_eq!(Solution::sub_array_ranges(nums), result);
    }

    #[test]
    fn test_2104_example_2() {
        let nums = vec![1, 3, 3];

        let result = 4;

        assert_eq!(Solution::sub_array_ranges(nums), result);
    }

    #[test]
    fn test_2104_example_3() {
        let nums = vec![4, -2, -3, 4, 1];

        let result = 59;

        assert_eq!(Solution::sub_array_ranges(nums), result);
    }
}
