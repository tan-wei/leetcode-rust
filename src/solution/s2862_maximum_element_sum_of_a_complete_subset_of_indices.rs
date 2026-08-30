/**
 * [2862] Maximum Element-Sum of a Complete Subset of Indices
 *
 * You are given a 1-indexed array nums. Your task is to select a complete subset from nums where every pair of selected indices multiplied is a <span data-keyword="perfect-square">perfect square,</span>. i. e. if you select ai and aj, i * j must be a perfect square.
 * Return the sum of the complete subset with the maximum sum.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [8,7,3,5,7,2,4,9]</span>
 * Output: <span class="example-io">16</span>
 * Explanation:
 * We select elements at indices 2 and 8 and 2 * 8 is a perfect square.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [8,10,3,8,1,13,7,9,4]</span>
 * Output: <span class="example-io">20</span>
 * Explanation:
 * We select elements at indices 1, 4, and 9. 1 * 4, 1 * 9, 4 * 9 are perfect squares.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= n == nums.length <= 10^4
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/
// discuss: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {
        0i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2862_example_1() {
        let nums = vec![8, 7, 3, 5, 7, 2, 4, 9];

        let result = 16;

        assert_eq!(Solution::maximum_sum(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2862_example_2() {
        let nums = vec![8, 10, 3, 8, 1, 13, 7, 9, 4];

        let result = 20;

        assert_eq!(Solution::maximum_sum(nums), result);
    }
}
