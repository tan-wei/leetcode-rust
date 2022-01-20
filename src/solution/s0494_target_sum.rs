/**
 * [0494] Target Sum
 *
 * You are given an integer array nums and an integer target.
 * You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
 *
 * 	For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
 *
 * Return the number of different expressions that you can build, which evaluates to target.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,1,1,1], target = 3
 * Output: 5
 * Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
 * -1 + 1 + 1 + 1 + 1 = 3
 * +1 - 1 + 1 + 1 + 1 = 3
 * +1 + 1 - 1 + 1 + 1 = 3
 * +1 + 1 + 1 - 1 + 1 = 3
 * +1 + 1 + 1 + 1 - 1 = 3
 *
 * Example 2:
 *
 * Input: nums = [1], target = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 20
 * 	0 <= nums[i] <= 1000
 * 	0 <= sum(nums[i]) <= 1000
 * 	-1000 <= target <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/target-sum/
// discuss: https://leetcode.com/problems/target-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let result = &mut 0i32;
        Self::dfs_helper(&nums, 0, 0, target, result);
        *result
    }

    fn dfs_helper(nums: &Vec<i32>, sum: i32, i: usize, target: i32, result: &mut i32) {
        if i >= nums.len() {
            if sum == target {
                *result += 1;
            }
            return;
        }

        Self::dfs_helper(nums, sum + nums[i], i + 1, target, result);
        Self::dfs_helper(nums, sum - nums[i], i + 1, target, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0494_example_1() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 3;
        let result = 5;

        assert_eq!(Solution::find_target_sum_ways(nums, target), result);
    }

    #[test]
    fn test_0494_example_2() {
        let nums = vec![1];
        let target = 1;
        let result = 1;

        assert_eq!(Solution::find_target_sum_ways(nums, target), result);
    }
}
