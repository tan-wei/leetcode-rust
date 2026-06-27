/**
 * [2521] Distinct Prime Factors of Product of Array
 *
 * Given an array of positive integers nums, return the number of distinct prime factors in the product of the elements of nums.
 * Note that:
 *
 * 	A number greater than 1 is called prime if it is divisible by only 1 and itself.
 * 	An integer val1 is a factor of another integer val2 if val2 / val1 is an integer.
 *
 *  
 * Example 1:
 *
 * Input: nums = [2,4,3,7,10,6]
 * Output: 4
 * Explanation:
 * The product of all the elements in nums is: 2 * 4 * 3 * 7 * 10 * 6 = 10080 = 2^5 * 3^2 * 5 * 7.
 * There are 4 distinct prime factors so we return 4.
 *
 * Example 2:
 *
 * Input: nums = [2,4,8,16]
 * Output: 1
 * Explanation:
 * The product of all the elements in nums is: 2 * 4 * 8 * 16 = 1024 = 2^10.
 * There is 1 distinct prime factor so we return 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	2 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-prime-factors-of-product-of-array/
// discuss: https://leetcode.com/problems/distinct-prime-factors-of-product-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2521_example_1() {
        let nums = vec![2, 4, 3, 7, 10, 6];

        let result = 4;

        assert_eq!(Solution::distinct_prime_factors(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2521_example_2() {
        let nums = vec![2, 4, 8, 16];

        let result = 1;

        assert_eq!(Solution::distinct_prime_factors(nums), result);
    }
}
