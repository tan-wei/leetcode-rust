/**
 * [1674] Minimum Moves to Make Array Complementary
 *
 * You are given an integer array nums of even length n and an integer limit. In one move, you can replace any integer from nums with another integer between 1 and limit, inclusive.
 * The array nums is complementary if for all indices i (0-indexed), nums[i] + nums[n - 1 - i] equals the same number. For example, the array [1,2,3,4] is complementary because for all indices i, nums[i] + nums[n - 1 - i] = 5.
 * Return the minimum number of moves required to make nums complementary.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,4,3], limit = 4
 * Output: 1
 * Explanation: In 1 move, you can change nums to [1,2,<u>2</u>,3] (underlined elements are changed).
 * nums[0] + nums[3] = 1 + 3 = 4.
 * nums[1] + nums[2] = 2 + 2 = 4.
 * nums[2] + nums[1] = 2 + 2 = 4.
 * nums[3] + nums[0] = 3 + 1 = 4.
 * Therefore, nums[i] + nums[n-1-i] = 4 for every i, so nums is complementary.
 *
 * Example 2:
 *
 * Input: nums = [1,2,2,1], limit = 2
 * Output: 2
 * Explanation: In 2 moves, you can change nums to [<u>2</u>,2,2,<u>2</u>]. You cannot change any number to 3 since 3 > limit.
 *
 * Example 3:
 *
 * Input: nums = [1,2,1,2], limit = 2
 * Output: 0
 * Explanation: nums is already complementary.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	2 <= n <= 10^5
 * 	1 <= nums[i] <= limit <= 10^5
 * 	n is even.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-make-array-complementary/
// discuss: https://leetcode.com/problems/minimum-moves-to-make-array-complementary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let n = nums.len();
        let mut arr = vec![0; 2 * limit + 2];

        for i in 0..n / 2 {
            let (a, b) = (nums[i] as usize, nums[n - i - 1] as usize);
            arr[2] += 2;
            arr[a.min(b) + 1] -= 1;
            arr[a + b] -= 1;
            arr[a + b + 1] += 1;
            arr[a.max(b) + limit + 1] += 1;
        }

        let (mut result, mut curr) = (i32::MAX, 0);

        for i in 2..arr.len() {
            curr += arr[i];
            if curr < result {
                result = curr;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_1674_example_1() {
        let nums = vec![1, 2, 4, 3];
        let limit = 4;

        let result = 1;

        assert_eq!(Solution::min_moves(nums, limit), result);
    }

    #[test]
    fn test_1674_example_2() {
        let nums = vec![1, 2, 2, 1];
        let limit = 2;

        let result = 2;

        assert_eq!(Solution::min_moves(nums, limit), result);
    }

    #[test]
    fn test_1674_example_3() {
        let nums = vec![1, 2, 1, 2];
        let limit = 2;

        let result = 0;

        assert_eq!(Solution::min_moves(nums, limit), result);
    }
}
