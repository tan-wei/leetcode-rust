/**
 * [1856] Maximum Subarray Min-Product
 *
 * The min-product of an array is equal to the minimum value in the array multiplied by the array's sum.
 *
 * 	For example, the array [3,2,5] (minimum value is 2) has a min-product of 2 * (3+2+5) = 2 * 10 = 20.
 *
 * Given an array of integers nums, return the maximum min-product of any non-empty subarray of nums. Since the answer may be large, return it modulo 10^9 + 7.
 * Note that the min-product should be maximized before performing the modulo operation. Testcases are generated such that the maximum min-product without modulo will fit in a 64-bit signed integer.
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,<u>2,3,2</u>]
 * Output: 14
 * Explanation: The maximum min-product is achieved with the subarray [2,3,2] (minimum value is 2).
 * 2 * (2+3+2) = 2 * 7 = 14.
 *
 * Example 2:
 *
 * Input: nums = [2,<u>3,3</u>,1,2]
 * Output: 18
 * Explanation: The maximum min-product is achieved with the subarray [3,3] (minimum value is 3).
 * 3 * (3+3) = 3 * 6 = 18.
 *
 * Example 3:
 *
 * Input: nums = [3,1,<u>5,6,4</u>,2]
 * Output: 60
 * Explanation: The maximum min-product is achieved with the subarray [5,6,4] (minimum value is 4).
 * 4 * (5+6+4) = 4 * 15 = 60.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray-min-product/
// discuss: https://leetcode.com/problems/maximum-subarray-min-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1856_example_1() {
        let nums = vec![1, 2, 3, 2];

        let result = 14;

        assert_eq!(Solution::max_sum_min_product(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1856_example_2() {
        let nums = vec![2, 3, 3, 1, 2];

        let result = 18;

        assert_eq!(Solution::max_sum_min_product(nums), result);
    }

    #[test]
    #[ignore]
    fn test_1856_example_3() {
        let nums = vec![3, 1, 5, 6, 4, 2];

        let result = 60;

        assert_eq!(Solution::max_sum_min_product(nums), result);
    }
}
