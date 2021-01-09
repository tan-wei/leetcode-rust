/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * Example 1:
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Output: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 * Example 2:
 *
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 * Example 3:
 *
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^3
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        for (idx, &n) in nums.iter().enumerate() {
            let y = target - n;

            match hash_map.get(&y) {
                Some(&i) => return vec![i as i32, idx as i32],
                None => hash_map.insert(n, idx as i32),
            };
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        assert_eq!(Solution::two_sum(nums, 9), vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let nums: Vec<i32> = vec![3, 2, 4];
        assert_eq!(Solution::two_sum(nums, 6), vec![1, 2]);
    }

    #[test]
    fn test_example_3() {
        let nums: Vec<i32> = vec![3, 3];
        assert_eq!(Solution::two_sum(nums, 6), vec![0, 1]);
    }

    #[test]
    fn test_empty_vec() {
        let nums: Vec<i32> = vec![];
        assert_eq!(nums, vec![]);
    }
}
