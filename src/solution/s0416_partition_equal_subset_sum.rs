/**
 * [0416] Partition Equal Subset Sum
 *
 * Given a non-empty array nums containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.
 *  
 * Example 1:
 *
 * Input: nums = [1,5,11,5]
 * Output: true
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,5]
 * Output: false
 * Explanation: The array cannot be partitioned into equal sum subsets.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	1 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-equal-subset-sum/
// discuss: https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        let max_idx = target as usize;
        let mut dp = vec![false; max_idx + 1];
        dp[0] = true;
        for n in nums {
            for j in (1..=max_idx).rev() {
                if j >= n as usize && !dp[j] {
                    dp[j] = dp[j - n as usize]
                }
            }
        }
        dp[max_idx]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0416_example_1() {
        let nums = vec![1, 5, 11, 5];
        let result = true;

        assert_eq!(Solution::can_partition(nums), result);
    }

    #[test]
    fn test_0416_example_2() {
        let nums = vec![1, 2, 3, 5];
        let result = false;

        assert_eq!(Solution::can_partition(nums), result);
    }
}
