/**
 * [0740] Delete and Earn
 *
 * You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:
 *
 * 	Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
 *
 * Return the maximum number of points you can earn by applying the above operation some number of times.
 *  
 * Example 1:
 *
 * Input: nums = [3,4,2]
 * Output: 6
 * Explanation: You can perform the following operations:
 * - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
 * - Delete 2 to earn 2 points. nums = [].
 * You earn a total of 6 points.
 *
 * Example 2:
 *
 * Input: nums = [2,2,3,3,3,4]
 * Output: 9
 * Explanation: You can perform the following operations:
 * - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
 * - Delete a 3 again to earn 3 points. nums = [3].
 * - Delete a 3 once more to earn 3 points. nums = [].
 * You earn a total of 9 points.
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-and-earn/
// discuss: https://leetcode.com/problems/delete-and-earn/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/delete-and-earn/discuss/1823126/Rust-or-0ms-or-2.1mb-or
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let (mut sum, mut dp) = (vec![0; 10001], vec![0; 10001]);
        nums.iter().for_each(|i| sum[*i as usize] += i);
        dp[1] = sum[1];
        (2..10001).for_each(|i| dp[i] = i32::max((sum[i] + dp[i - 2]), dp[i - 1]));
        dp[10001 - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0740_example_1() {
        let nums = vec![3, 4, 2];
        let result = 6;

        assert_eq!(Solution::delete_and_earn(nums), result);
    }

    #[test]
    fn test_0740_example_2() {
        let nums = vec![2, 2, 3, 3, 3, 4];
        let result = 9;

        assert_eq!(Solution::delete_and_earn(nums), result);
    }
}
