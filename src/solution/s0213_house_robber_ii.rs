/**
 * [213] House Robber II
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
 * Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,2]
 * Output: 3
 * Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 * Example 3:
 *
 * Input: nums = [0]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber-ii/
// discuss: https://leetcode.com/problems/house-robber-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        match n {
            0 => 0,
            1 => nums[0],
            n => std::cmp::max(Self::helper(&nums, 0, n - 2), Self::helper(&nums, 1, n - 1)),
        }
    }

    fn helper(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        let mut pre = 0;
        let mut cur = 0;
        for i in l..=r {
            let mut temp = std::cmp::max(pre + nums[i], cur);
            pre = cur;
            cur = temp;
        }
        cur
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0213_example_1() {
        let nums = vec![2, 3, 2];
        let result = 3;

        assert_eq!(Solution::rob(nums), result);
    }

    #[test]
    fn test_0213_example_2() {
        let nums = vec![1, 2, 3, 1];
        let result = 4;

        assert_eq!(Solution::rob(nums), result);
    }

    #[test]
    fn test_0213_example_3() {
        let nums = vec![0];
        let result = 0;

        assert_eq!(Solution::rob(nums), result);
    }
}
