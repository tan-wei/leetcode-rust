/**
 * [1793] Maximum Score of a Good Subarray
 *
 * You are given an array of integers nums (0-indexed) and an integer k.
 * The score of a subarray (i, j) is defined as min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1). A good subarray is a subarray where i <= k <= j.
 * Return the maximum possible score of a good subarray.
 *  
 * Example 1:
 *
 * Input: nums = [1,4,3,7,4,5], k = 3
 * Output: 15
 * Explanation: The optimal subarray is (1, 5) with a score of min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15.
 *
 * Example 2:
 *
 * Input: nums = [5,5,4,5,4,1,1,1], k = 0
 * Output: 20
 * Explanation: The optimal subarray is (0, 4) with a score of min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 2 * 10^4
 * 	0 <= k < nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-of-a-good-subarray/
// discuss: https://leetcode.com/problems/maximum-score-of-a-good-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1793_example_1() {
        let nums = vec![1, 4, 3, 7, 4, 5];
        let k = 3;

        let result = 15;

        assert_eq!(Solution::maximum_score(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_1793_example_2() {
        let nums = vec![5, 5, 4, 5, 4, 1, 1, 1];
        let k = 0;

        let result = 20;

        assert_eq!(Solution::maximum_score(nums, k), result);
    }
}
