/**
 * [2717] Semi-Ordered Permutation
 *
 * You are given a 0-indexed permutation of n integers nums.
 * A permutation is called semi-ordered if the first number equals 1 and the last number equals n. You can perform the below operation as many times as you want until you make nums a semi-ordered permutation:
 *
 * 	Pick two adjacent elements in nums, then swap them.
 *
 * Return the minimum number of operations to make nums a semi-ordered permutation.
 * A permutation is a sequence of integers from 1 to n of length n containing each number exactly once.
 *  
 * Example 1:
 *
 * Input: nums = [2,1,4,3]
 * Output: 2
 * Explanation: We can make the permutation semi-ordered using these sequence of operations:
 * 1 - swap i = 0 and j = 1. The permutation becomes [1,2,4,3].
 * 2 - swap i = 2 and j = 3. The permutation becomes [1,2,3,4].
 * It can be proved that there is no sequence of less than two operations that make nums a semi-ordered permutation.
 *
 * Example 2:
 *
 * Input: nums = [2,4,1,3]
 * Output: 3
 * Explanation: We can make the permutation semi-ordered using these sequence of operations:
 * 1 - swap i = 1 and j = 2. The permutation becomes [2,1,4,3].
 * 2 - swap i = 0 and j = 1. The permutation becomes [1,2,4,3].
 * 3 - swap i = 2 and j = 3. The permutation becomes [1,2,3,4].
 * It can be proved that there is no sequence of less than three operations that make nums a semi-ordered permutation.
 *
 * Example 3:
 *
 * Input: nums = [1,3,4,2,5]
 * Output: 0
 * Explanation: The permutation is already a semi-ordered permutation.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length == n <= 50
 * 	1 <= nums[i] <= 50
 * 	nums is a permutation.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/semi-ordered-permutation/
// discuss: https://leetcode.com/problems/semi-ordered-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        nums.iter().zip(1..).fold(0, |acc, (&n, i)| match n {
            1 => acc + i - 1 - i32::from(acc != 0),
            x if x == nums.len() as i32 => acc + nums.len() as i32 - i,
            _ => acc,
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2717_example_1() {
        let nums = vec![2, 1, 4, 3];

        let result = 2;

        assert_eq!(Solution::semi_ordered_permutation(nums), result);
    }

    #[test]
    fn test_2717_example_2() {
        let nums = vec![2, 4, 1, 3];

        let result = 3;

        assert_eq!(Solution::semi_ordered_permutation(nums), result);
    }

    #[test]
    fn test_2717_example_3() {
        let nums = vec![1, 3, 4, 2, 5];

        let result = 0;

        assert_eq!(Solution::semi_ordered_permutation(nums), result);
    }
}
