/**
 * [2598] Smallest Missing Non-negative Integer After Operations
 *
 * You are given a 0-indexed integer array nums and an integer value.
 * In one operation, you can add or subtract value from any element of nums.
 *
 * 	For example, if nums = [1,2,3] and value = 2, you can choose to subtract value from nums[0] to make nums = [-1,2,3].
 *
 * The MEX (minimum excluded) of an array is the smallest missing non-negative integer in it.
 *
 * 	For example, the MEX of [-1,2,3] is 0 while the MEX of [1,0,3] is 2.
 *
 * Return the maximum MEX of nums after applying the mentioned operation any number of times.
 *  
 * Example 1:
 *
 * Input: nums = [1,-10,7,13,6,8], value = 5
 * Output: 4
 * Explanation: One can achieve this result by applying the following operations:
 * - Add value to nums[1] twice to make nums = [1,0,7,13,6,8]
 * - Subtract value from nums[2] once to make nums = [1,0,2,13,6,8]
 * - Subtract value from nums[3] twice to make nums = [1,0,2,3,6,8]
 * The MEX of nums is 4. It can be shown that 4 is the maximum MEX we can achieve.
 *
 * Example 2:
 *
 * Input: nums = [1,-10,7,13,6,8], value = 7
 * Output: 2
 * Explanation: One can achieve this result by applying the following operation:
 * - subtract value from nums[2] once to make nums = [1,-10,0,13,6,8]
 * The MEX of nums is 2. It can be shown that 2 is the maximum MEX we can achieve.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, value <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/
// discuss: https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        nums.into_iter()
            .fold(vec![0; value as usize], |mut f, n| {
                f[n.rem_euclid(value) as usize] += 1;
                f
            })
            .into_iter()
            .zip(0..)
            .min_by_key(|(f, _)| *f)
            .map(|(f, i)| f * value + i)
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2598_example_1() {
        let nums = vec![1, -10, 7, 13, 6, 8];
        let value = 5;

        let result = 4;

        assert_eq!(Solution::find_smallest_integer(nums, value), result);
    }

    #[test]
    fn test_2598_example_2() {
        let nums = vec![1, -10, 7, 13, 6, 8];
        let value = 7;

        let result = 2;

        assert_eq!(Solution::find_smallest_integer(nums, value), result);
    }
}
