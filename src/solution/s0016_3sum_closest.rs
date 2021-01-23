/**
 * [16] 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *  
 * Example 1:
 *
 * Input: nums = [-1,2,1,-4], target = 1
 * Output: 2
 * Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 10^3
 * 	-10^3 <= nums[i] <= 10^3
 * 	-10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 3 {
            return nums.iter().sum();
        }

        let mut nums = nums;
        let mut result = nums[0] + nums[1] + nums[2];

        nums.sort();

        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];

                if sum == target {
                    return target;
                }

                if (sum - target).abs() < (result - target).abs() {
                    result = sum;
                }

                if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
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
    fn test_0016_example_1() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;

        assert_eq!(Solution::three_sum_closest(nums, target), 2);
    }
}
