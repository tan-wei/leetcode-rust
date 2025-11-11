/**
 * [2233] Maximum Product After K Increments
 *
 * You are given an array of non-negative integers nums and an integer k. In one operation, you may choose any element from nums and increment it by 1.
 * Return the maximum product of nums after at most k operations. Since the answer may be very large, return it modulo 10^9 + 7. Note that you should maximize the product before taking the modulo.
 *  
 * Example 1:
 *
 * Input: nums = [0,4], k = 5
 * Output: 20
 * Explanation: Increment the first number 5 times.
 * Now nums = [5, 4], with a product of 5 * 4 = 20.
 * It can be shown that 20 is maximum product possible, so we return 20.
 * Note that there may be other ways to increment nums to have the maximum product.
 *
 * Example 2:
 *
 * Input: nums = [6,3,3,2], k = 2
 * Output: 216
 * Explanation: Increment the second number 1 time and increment the fourth number 1 time.
 * Now nums = [6, 4, 3, 3], with a product of 6 * 4 * 3 * 3 = 216.
 * It can be shown that 216 is maximum product possible, so we return 216.
 * Note that there may be other ways to increment nums to have the maximum product.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, k <= 10^5
 * 	0 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-after-k-increments/
// discuss: https://leetcode.com/problems/maximum-product-after-k-increments/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2233_example_1() {
        let nums = vec![0, 4];
        let k = 5;

        let result = 20;

        assert_eq!(Solution::maximum_product(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2233_example_2() {
        let nums = vec![6, 3, 3, 2];
        let k = 2;

        let result = 216;

        assert_eq!(Solution::maximum_product(nums, k), result);
    }
}
