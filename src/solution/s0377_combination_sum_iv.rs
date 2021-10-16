/**
 * [377] Combination Sum IV
 *
 * Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3], target = 4
 * Output: 7
 * Explanation:
 * The possible combination ways are:
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 * Note that different sequences are counted as different combinations.
 *
 * Example 2:
 *
 * Input: nums = [9], target = 3
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	1 <= nums[i] <= 1000
 * 	All the elements of nums are unique.
 * 	1 <= target <= 1000
 *
 *  
 * Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iv/
// discuss: https://leetcode.com/problems/combination-sum-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1usize..=target as usize {
            for num in nums.iter() {
                if *num as usize <= i {
                    dp[i] += dp[i - *num as usize];
                }
            }
        }

        dp[target as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0377_example_1() {
        let nums = vec![1, 2, 3];
        let target = 4;
        let result = 7;

        assert_eq!(Solution::combination_sum4(nums, target), result);
    }

    #[test]
    fn test_0377_example_2() {
        let nums = vec![9];
        let target = 3;
        let result = 0;

        assert_eq!(Solution::combination_sum4(nums, target), result);
    }
}
