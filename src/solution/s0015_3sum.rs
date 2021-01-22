/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 * Notice that the solution set must not contain duplicate triplets.
 *  
 * Example 1:
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Example 2:
 * Input: nums = []
 * Output: []
 * Example 3:
 * Input: nums = [0]
 * Output: []
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 3000
 * 	-10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 2 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        let mut result = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                return result;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let l = nums[left];
                let r = nums[right];
                let t = nums[i];
                let total = l + r + t;
                if total > 0 {
                    right -= 1;
                } else if total < 0 {
                    left += 1;
                } else {
                    result.push(vec![l, r, t]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
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
    use std::collections::HashSet;

    #[test]
    fn test_0015_example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = vec![vec![-1, 2, -1], vec![0, 1, -1]];

        assert_eq!(Solution::three_sum(nums), result);
    }

    #[test]
    fn test_0015_example_2() {
        let nums = vec![];
        let result: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::three_sum(nums), result);
    }

    #[test]
    fn test_0015_example_3() {
        let nums = vec![0];
        let result: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::three_sum(nums), result);
    }
}
