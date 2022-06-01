/**
 * [0698] Partition to K Equal Sum Subsets
 *
 * Given an integer array nums and an integer k, return true if it is possible to divide this array into k non-empty subsets whose sums are all equal.
 *  
 * Example 1:
 *
 * Input: nums = [4,3,2,3,5,2,1], k = 4
 * Output: true
 * Explanation: It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4], k = 3
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 16
 * 	1 <= nums[i] <= 10^4
 * 	The frequency of each element is in the range [1, 4].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
// discuss: https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        match sum % k != 0 {
            true => false,
            false => Self::dfs_helper(&nums, 0, sum / k, 0, &mut vec![false; nums.len()]),
        }
    }

    fn dfs_helper(nums: &[i32], curr: i32, target: i32, i: usize, used: &mut Vec<bool>) -> bool {
        if used.iter().all(|&b| b) {
            return true;
        }

        for j in i..nums.len() {
            if used[j] || curr + nums[j] > target {
                continue;
            }
            let next = (curr + nums[j]) % target;
            used[j] = true;
            if Self::dfs_helper(nums, next, target, if next == 0 { 0 } else { j + 1 }, used) {
                return true;
            }
            used[j] = false;
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0698_example_1() {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        let result = true;

        assert_eq!(Solution::can_partition_k_subsets(nums, k), result);
    }

    #[test]
    fn test_0698_example_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        let result = false;

        assert_eq!(Solution::can_partition_k_subsets(nums, k), result);
    }
}
