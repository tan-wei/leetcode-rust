/**
 * [1027] Longest Arithmetic Subsequence
 *
 * Given an array nums of integers, return the length of the longest arithmetic subsequence in nums.
 * Note that:
 *
 * 	A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 * 	A sequence seq is arithmetic if seq[i + 1] - seq[i] are all the same value (for 0 <= i < seq.length - 1).
 *
 *  
 * Example 1:
 *
 * Input: nums = [3,6,9,12]
 * Output: 4
 * Explanation:  The whole array is an arithmetic sequence with steps of length = 3.
 *
 * Example 2:
 *
 * Input: nums = [9,4,7,2,10]
 * Output: 3
 * Explanation:  The longest arithmetic subsequence is [4,7,10].
 *
 * Example 3:
 *
 * Input: nums = [20,1,15,3,10,5,8]
 * Output: 4
 * Explanation:  The longest arithmetic subsequence is [20,15,10,5].
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 1000
 * 	0 <= nums[i] <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-arithmetic-subsequence/
// discuss: https://leetcode.com/problems/longest-arithmetic-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut result = 2;

        let mut dp = std::collections::HashMap::new();
        for j in 0..nums.len() {
            for i in 0..j {
                let diff = nums[j] - nums[i];
                let val = *dp.get(&(i, diff)).unwrap_or(&1);
                dp.insert((j, diff), val + 1);

                result = std::cmp::max(result, val + 1);
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
    fn test_1027_example_1() {
        let nums = vec![3, 6, 9, 12];
        let result = 4;

        assert_eq!(Solution::longest_arith_seq_length(nums), result);
    }

    #[test]
    fn test_1027_example_2() {
        let nums = vec![9, 4, 7, 2, 10];
        let result = 3;

        assert_eq!(Solution::longest_arith_seq_length(nums), result);
    }

    #[test]
    fn test_1027_example_3() {
        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        let result = 4;

        assert_eq!(Solution::longest_arith_seq_length(nums), result);
    }
}
