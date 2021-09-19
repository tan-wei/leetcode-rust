/**
 * [334] Increasing Triplet Subsequence
 *
 * Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: true
 * Explanation: Any triplet where i < j < k is valid.
 *
 * Example 2:
 *
 * Input: nums = [5,4,3,2,1]
 * Output: false
 * Explanation: No triplet exists.
 *
 * Example 3:
 *
 * Input: nums = [2,1,5,0,4,6]
 * Output: true
 * Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/increasing-triplet-subsequence/
// discuss: https://leetcode.com/problems/increasing-triplet-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut candidate1 = i32::MAX;
        let mut candidate2 = i32::MAX;

        for num in nums.into_iter() {
            if num <= candidate1 {
                candidate1 = num;
            } else if num <= candidate2 {
                candidate2 = num;
            } else {
                // so candidate1 < candidate2 < num
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0334_example_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = true;

        assert_eq!(Solution::increasing_triplet(nums), result);
    }

    #[test]
    fn test_0334_example_2() {
        let nums = vec![5, 4, 3, 2, 1];
        let result = false;

        assert_eq!(Solution::increasing_triplet(nums), result);
    }

    #[test]
    fn test_0334_example_3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let result = true;

        assert_eq!(Solution::increasing_triplet(nums), result);
    }
}
