/**
 * [2592] Maximize Greatness of an Array
 *
 * You are given a 0-indexed integer array nums. You are allowed to permute nums into a new array perm of your choosing.
 * We define the greatness of nums be the number of indices 0 <= i < nums.length for which perm[i] > nums[i].
 * Return the maximum possible greatness you can achieve after permuting nums.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,5,2,1,3,1]
 * Output: 4
 * Explanation: One of the optimal rearrangements is perm = [2,5,1,3,3,1,1].
 * At indices = 0, 1, 3, and 4, perm[i] > nums[i]. Hence, we return 4.
 * Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: 3
 * Explanation: We can prove the optimal perm is [2,3,4,1].
 * At indices = 0, 1, and 2, perm[i] > nums[i]. Hence, we return 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-greatness-of-an-array/
// discuss: https://leetcode.com/problems/maximize-greatness-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2592_example_1() {
        let nums = vec![1, 3, 5, 2, 1, 3, 1];

        let result = 4;

        assert_eq!(Solution::maximize_greatness(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2592_example_2() {
        let nums = vec![1, 2, 3, 4];

        let result = 3;

        assert_eq!(Solution::maximize_greatness(nums), result);
    }
}
