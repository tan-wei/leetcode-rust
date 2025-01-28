/**
 * [1862] Sum of Floored Pairs
 *
 * Given an integer array nums, return the sum of floor(nums[i] / nums[j]) for all pairs of indices 0 <= i, j < nums.length in the array. Since the answer may be too large, return it modulo 10^9 + 7.
 * The floor() function returns the integer part of the division.
 *  
 * Example 1:
 *
 * Input: nums = [2,5,9]
 * Output: 10
 * Explanation:
 * floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
 * floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
 * floor(5 / 2) = 2
 * floor(9 / 2) = 4
 * floor(9 / 5) = 1
 * We calculate the floor of the division for every pair of indices in the array then sum them up.
 *
 * Example 2:
 *
 * Input: nums = [7,7,7,7,7,7,7]
 * Output: 49
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-floored-pairs/
// discuss: https://leetcode.com/problems/sum-of-floored-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1862_example_1() {
        let nums = vec![2, 5, 9];

        let result = 10;

        assert_eq!(Solution::sum_of_floored_pairs(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1862_example_2() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];

        let result = 49;

        assert_eq!(Solution::sum_of_floored_pairs(nums), result);
    }
}
