/**
 * [1673] Find the Most Competitive Subsequence
 *
 * Given an integer array nums and a positive integer k, return the most competitive subsequence of nums of size k.
 * An array's subsequence is a resulting sequence obtained by erasing some (possibly zero) elements from the array.
 * We define that a subsequence a is more competitive than a subsequence b (of the same length) if in the first position where a and b differ, subsequence a has a number less than the corresponding number in b. For example, [1,3,4] is more competitive than [1,3,5] because the first position they differ is at the final number, and 4 is less than 5.
 *  
 * Example 1:
 *
 * Input: nums = [3,5,2,6], k = 2
 * Output: [2,6]
 * Explanation: Among the set of every possible subsequence: {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]}, [2,6] is the most competitive.
 *
 * Example 2:
 *
 * Input: nums = [2,4,3,3,5,4,9,6], k = 4
 * Output: [2,3,3,4]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 * 	1 <= k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-most-competitive-subsequence/
// discuss: https://leetcode.com/problems/find-the-most-competitive-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-the-most-competitive-subsequence/solutions/1028631/rust-stack-solution/
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = Vec::with_capacity(k as usize);
        for (i, &num) in nums.iter().enumerate() {
            while let Some(&last) = stack.last() {
                if num < last && stack.len() + nums.len() - i > k as usize {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() < k as usize {
                stack.push(num);
            }
        }
        stack
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1673_example_1() {
        let nums = vec![3, 5, 2, 6];
        let k = 2;

        let result = vec![2, 6];

        assert_eq!(Solution::most_competitive(nums, k), result);
    }

    #[test]
    fn test_1673_example_2() {
        let nums = vec![2, 4, 3, 3, 5, 4, 9, 6];
        let k = 4;

        let result = vec![2, 3, 3, 4];

        assert_eq!(Solution::most_competitive(nums, k), result);
    }
}
