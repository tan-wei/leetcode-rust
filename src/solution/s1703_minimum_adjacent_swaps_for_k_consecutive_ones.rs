/**
 * [1703] Minimum Adjacent Swaps for K Consecutive Ones
 *
 * You are given an integer array, nums, and an integer k. nums comprises of only 0's and 1's. In one move, you can choose two adjacent indices and swap their values.
 * Return the minimum number of moves required so that nums has k consecutive 1's.
 *  
 * Example 1:
 *
 * Input: nums = [1,0,0,1,0,1], k = 2
 * Output: 1
 * Explanation: In 1 move, nums could be [1,0,0,0,<u>1</u>,<u>1</u>] and have 2 consecutive 1's.
 *
 * Example 2:
 *
 * Input: nums = [1,0,0,0,0,0,1,1], k = 3
 * Output: 5
 * Explanation: In 5 moves, the leftmost 1 can be shifted right until nums = [0,0,0,0,0,<u>1</u>,<u>1</u>,<u>1</u>].
 *
 * Example 3:
 *
 * Input: nums = [1,1,0,1], k = 2
 * Output: 0
 * Explanation: nums already has 2 consecutive 1's.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is 0 or 1.
 * 	1 <= k <= sum(nums)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/
// discuss: https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut a = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                a.push(i as i64);
            }
        }

        let mut b = vec![0];

        for &num in a.iter() {
            b.push(b.last().unwrap() + num);
        }

        let mut result = 2e9 as i64;

        for i in 0..a.len() - k + 1 {
            result = result.min(b[i + k] - b[k / 2 + i] - b[(k + 1) / 2 + i] + b[i]);
        }

        result -= ((k / 2) * ((k + 1) / 2)) as i64;

        result as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1703_example_1() {
        let nums = vec![1, 0, 0, 1, 0, 1];
        let k = 2;

        let result = 1;

        assert_eq!(Solution::min_moves(nums, k), result);
    }

    #[test]
    fn test_1703_example_2() {
        let nums = vec![1, 0, 0, 0, 0, 0, 1, 1];
        let k = 3;

        let result = 5;

        assert_eq!(Solution::min_moves(nums, k), result);
    }

    #[test]
    fn test_1703_example_3() {
        let nums = vec![1, 1, 0, 1];
        let k = 2;

        let result = 0;

        assert_eq!(Solution::min_moves(nums, k), result);
    }
}
