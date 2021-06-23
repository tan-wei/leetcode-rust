/**
 * [198] House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
 * Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 * Example 2:
 *
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 400
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber/
// discuss: https://leetcode.com/problems/house-robber/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => std::cmp::max(nums[0], nums[1]),
            n => {
                let mut f = vec![0; n];
                f[0] = nums[0];
                f[1] = nums[0].max(nums[1]);
                for i in 2..nums.len() {
                    f[i] = f[i - 1].max(f[i - 2] + nums[i]);
                }
                f[n - 1]
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0198_example_1() {
        let nums = vec![1, 2, 3, 1];
        let result = 4;

        assert_eq!(Solution::rob(nums), result);
    }

    #[test]
    fn test_0198_example_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let result = 12;

        assert_eq!(Solution::rob(nums), result);
    }
}
