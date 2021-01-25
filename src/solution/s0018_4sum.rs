/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 * Notice that the solution set must not contain duplicate quadruplets.
 *  
 * Example 1:
 * Input: nums = [1,0,-1,0,-2,2], target = 0
 * Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 * Example 2:
 * Input: nums = [], target = 0
 * Output: []
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 200
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    // Brute force may be AC, O(n^4), about 100ms
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let target: i64 = target.into();
        let mut result: HashSet<[i32; 4]> = HashSet::new();
        for (i, &v0) in nums.iter().enumerate() {
            let sum = i64::from(v0);
            let subslice = &nums[i + 1..];
            for (i, &v1) in subslice.iter().enumerate() {
                let sum = sum + i64::from(v1);
                let subslice = &subslice[i + 1..];
                for (i, &v2) in subslice.iter().enumerate() {
                    let sum = sum + i64::from(v2);
                    let subslice = &subslice[i + 1..];
                    for (_, &v3) in subslice.iter().enumerate() {
                        let sum = sum + i64::from(v3);
                        if sum == target {
                            result.insert([v0, v1, v2, v3]);
                        }
                    }
                }
            }
        }
        result.into_iter().map(|v| v.to_vec()).collect()
    }

    pub fn four_sum_v2(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // O(n^3), about 8ms
        let mut nums = nums;
        nums.sort();
        let mut result = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let c_sum = nums[i] + nums[j];
                let mut start = j + 1;
                let mut end = nums.len() - 1;
                while start < end {
                    let total = c_sum + nums[start] + nums[end];
                    if target == total {
                        let new_vec = vec![nums[i], nums[j], nums[start], nums[end]];
                        if !result.contains(&new_vec) {
                            result.push(new_vec);
                        }
                        start += 1;
                        end -= 1;
                    } else if total < target {
                        start += 1;
                    } else {
                        end -= 1;
                    }
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
    fn test_0018_example_1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let result = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq_sorted!(Solution::four_sum(nums.clone(), target), result.clone());
        assert_eq_sorted!(Solution::four_sum_v2(nums.clone(), target), result.clone());
    }

    #[test]
    fn test_0018_example_2() {
        let nums: Vec<i32> = Vec::new();
        let target = 0;
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq_sorted!(Solution::four_sum(nums.clone(), target), result.clone());
        assert_eq_sorted!(Solution::four_sum_v2(nums.clone(), target), result.clone());
    }
}
