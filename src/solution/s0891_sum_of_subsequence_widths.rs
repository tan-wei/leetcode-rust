/**
 * [0891] Sum of Subsequence Widths
 *
 * The width of a sequence is the difference between the maximum and minimum elements in the sequence.
 * Given an array of integers nums, return the sum of the widths of all the non-empty subsequences of nums. Since the answer may be very large, return it modulo 10^9 + 7.
 * A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].
 *  
 * Example 1:
 *
 * Input: nums = [2,1,3]
 * Output: 6
 * Explanation: The subsequences are [1], [2], [3], [2,1], [2,3], [1,3], [2,1,3].
 * The corresponding widths are 0, 0, 0, 1, 1, 2, 2.
 * The sum of these widths is 6.
 *
 * Example 2:
 *
 * Input: nums = [2]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subsequence-widths/
// discuss: https://leetcode.com/problems/sum-of-subsequence-widths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-subsequence-widths/solutions/2791666/c-easy-shorts/
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut c = 1;
        let mut result: i64 = 0;
        let n = nums.len();

        for i in 0..n {
            result = (result + nums[i] as i64 * c - nums[n - i - 1] as i64 * c) % MOD;
            c = c * 2 % MOD;
        }

        ((result + MOD) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0891_example_1() {
        let nums = vec![2, 1, 3];
        let result = 6;

        assert_eq!(Solution::sum_subseq_widths(nums), result);
    }

    #[test]
    fn test_0891_example_2() {
        let nums = vec![2];
        let result = 0;

        assert_eq!(Solution::sum_subseq_widths(nums), result);
    }
}
