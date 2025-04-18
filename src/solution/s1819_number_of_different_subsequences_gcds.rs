/**
 * [1819] Number of Different Subsequences GCDs
 *
 * You are given an array nums that consists of positive integers.
 * The GCD of a sequence of numbers is defined as the greatest integer that divides all the numbers in the sequence evenly.
 *
 * 	For example, the GCD of the sequence [4,6,16] is 2.
 *
 * A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
 *
 * 	For example, [2,5,10] is a subsequence of [1,2,1,<u>2</u>,4,1,<u>5</u>,<u>10</u>].
 *
 * Return the number of different GCDs among all non-empty subsequences of nums.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/17/image-1.png" style="width: 149px; height: 309px;" />
 * Input: nums = [6,10,3]
 * Output: 5
 * Explanation: The figure shows all the non-empty subsequences and their GCDs.
 * The different GCDs are 6, 10, 3, 2, and 1.
 *
 * Example 2:
 *
 * Input: nums = [5,15,40,5,6]
 * Output: 7
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 2 * 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-different-subsequences-gcds/
// discuss: https://leetcode.com/problems/number-of-different-subsequences-gcds/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1819_example_1() {
        let nums = vec![6, 10, 3];

        let result = 5;

        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1819_example_2() {
        let nums = vec![5, 15, 40, 5, 6];

        let result = 7;

        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), result);
    }
}
