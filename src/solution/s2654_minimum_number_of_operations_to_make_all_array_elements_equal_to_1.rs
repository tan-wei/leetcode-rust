/**
 * [2654] Minimum Number of Operations to Make All Array Elements Equal to 1
 *
 * You are given a 0-indexed array nums consisting of positive integers. You can do the following operation on the array any number of times:
 *
 * 	Select an index i such that 0 <= i < n - 1 and replace either of nums[i] or nums[i+1] with their gcd value.
 *
 * Return the minimum number of operations to make all elements of nums equal to 1. If it is impossible, return -1.
 * The gcd of two integers is the greatest common divisor of the two integers.
 *  
 * Example 1:
 *
 * Input: nums = [2,6,3,4]
 * Output: 4
 * Explanation: We can do the following operations:
 * - Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
 * - Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
 * - Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
 * - Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].
 *
 * Example 2:
 *
 * Input: nums = [2,10,6,14]
 * Output: -1
 * Explanation: It can be shown that it is impossible to make all the elements equal to 1.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 50
 * 	1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2654_example_1() {
        let nums = vec![2, 6, 3, 4];

        let result = 4;

        assert_eq!(Solution::min_operations(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2654_example_2() {
        let nums = vec![2, 10, 6, 14];

        let result = -1;

        assert_eq!(Solution::min_operations(nums), result);
    }
}
