/**
 * [2035] Partition Array Into Two Arrays to Minimize Sum Difference
 *
 * You are given an integer array nums of 2 * n integers. You need to partition nums into two arrays of length n to minimize the absolute difference of the sums of the arrays. To partition nums, put each element of nums into one of the two arrays.
 * Return the minimum possible absolute difference.
 *  
 * Example 1:
 * <img alt="example-1" src="https://assets.leetcode.com/uploads/2021/10/02/ex1.png" style="width: 240px; height: 106px;" />
 * Input: nums = [3,9,7,3]
 * Output: 2
 * Explanation: One optimal partition is: [3,9] and [7,3].
 * The absolute difference between the sums of the arrays is abs((3 + 9) - (7 + 3)) = 2.
 *
 * Example 2:
 *
 * Input: nums = [-36,36]
 * Output: 72
 * Explanation: One optimal partition is: [-36] and [36].
 * The absolute difference between the sums of the arrays is abs((-36) - (36)) = 72.
 *
 * Example 3:
 * <img alt="example-3" src="https://assets.leetcode.com/uploads/2021/10/02/ex3.png" style="width: 316px; height: 106px;" />
 * Input: nums = [2,-1,0,4,-2,-9]
 * Output: 0
 * Explanation: One optimal partition is: [2,4,-9] and [-1,0,-2].
 * The absolute difference between the sums of the arrays is abs((2 + 4 + -9) - (-1 + 0 + -2)) = 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 15
 * 	nums.length == 2 * n
 * 	-10^7 <= nums[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/
// discuss: https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2035_example_1() {
        let nums = vec![3, 9, 7, 3];

        let result = 2;

        assert_eq!(Solution::minimum_difference(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2035_example_2() {
        let nums = vec![-36, 36];

        let result = 72;

        assert_eq!(Solution::minimum_difference(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2035_example_3() {
        let nums = vec![2, -1, 0, 4, -2, -9];

        let result = 0;

        assert_eq!(Solution::minimum_difference(nums), result);
    }
}
