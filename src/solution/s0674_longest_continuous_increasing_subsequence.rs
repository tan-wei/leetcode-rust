/**
 * [0674] Longest Continuous Increasing Subsequence
 *
 * Given an unsorted array of integers nums, return the length of the longest continuous increasing subsequence (i.e. subarray). The subsequence must be strictly increasing.
 * A continuous increasing subsequence is defined by two indices l and r (l < r) such that it is [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] and for each l <= i < r, nums[i] < nums[i + 1].
 *  
 * Example 1:
 *
 * Input: nums = [1,3,5,4,7]
 * Output: 3
 * Explanation: The longest continuous increasing subsequence is [1,3,5] with length 3.
 * Even though [1,3,5,7] is an increasing subsequence, it is not continuous as elements 5 and 7 are separated by element
 * 4.
 *
 * Example 2:
 *
 * Input: nums = [2,2,2,2,2]
 * Output: 1
 * Explanation: The longest continuous increasing subsequence is [2] with length 1. Note that it must be strictly
 * increasing.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-continuous-increasing-subsequence/
// discuss: https://leetcode.com/problems/longest-continuous-increasing-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut len = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] <= nums[i - 1] {
                len = 0;
            }
            len += 1;
            result = std::cmp::max(result, len);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0674_example_1() {
        let nums = vec![1, 3, 5, 4, 7];
        let result = 3;

        assert_eq!(Solution::find_length_of_lcis(nums), result);
    }

    #[test]
    fn test_0674_example_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let result = 1;

        assert_eq!(Solution::find_length_of_lcis(nums), result);
    }
}
