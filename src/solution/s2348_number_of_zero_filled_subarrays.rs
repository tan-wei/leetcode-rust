/**
 * [2348] Number of Zero-Filled Subarrays
 *
 * Given an integer array nums, return the number of subarrays filled with 0.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,0,0,2,0,0,4]
 * Output: 6
 * Explanation:
 * There are 4 occurrences of [0] as a subarray.
 * There are 2 occurrences of [0,0] as a subarray.
 * There is no occurrence of a subarray with a size more than 2 filled with 0. Therefore, we return 6.
 * Example 2:
 *
 * Input: nums = [0,0,0,2,0,0]
 * Output: 9
 * Explanation:
 * There are 5 occurrences of [0] as a subarray.
 * There are 3 occurrences of [0,0] as a subarray.
 * There is 1 occurrence of [0,0,0] as a subarray.
 * There is no occurrence of a subarray with a size more than 3 filled with 0. Therefore, we return 9.
 *
 * Example 3:
 *
 * Input: nums = [2,10,2019]
 * Output: 0
 * Explanation: There is no subarray filled with 0. Therefore, we return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-zero-filled-subarrays/
// discuss: https://leetcode.com/problems/number-of-zero-filled-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        nums.iter()
            .fold((1, 0), |(acc, sum), &n| {
                if n == 0 {
                    (acc + 1, sum + acc)
                } else {
                    (1, sum)
                }
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2348_example_1() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];

        let result = 6;

        assert_eq!(Solution::zero_filled_subarray(nums), result);
    }

    #[test]
    fn test_2348_example_2() {
        let nums = vec![0, 0, 0, 2, 0, 0];

        let result = 9;

        assert_eq!(Solution::zero_filled_subarray(nums), result);
    }

    #[test]
    fn test_2348_example_3() {
        let nums = vec![2, 10, 2019];

        let result = 0;

        assert_eq!(Solution::zero_filled_subarray(nums), result);
    }
}
