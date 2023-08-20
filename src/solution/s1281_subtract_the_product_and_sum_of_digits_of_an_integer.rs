/**
 * [1281] Subtract the Product and Sum of Digits of an Integer
 *
 * Given an integer number n, return the difference between the product of its digits and the sum of its digits.
 *
 * Example 1:
 *
 * Input: n = 234
 * Output: 15
 * Explanation:
 * Product of digits = 2 * 3 * 4 = 24
 * Sum of digits = 2 + 3 + 4 = 9
 * Result = 24 - 9 = 15
 *
 * Example 2:
 *
 * Input: n = 4421
 * Output: 21
 * Explanation:
 * Product of digits = 4 * 4 * 2 * 1 = 32
 * Sum of digits = 4 + 4 + 2 + 1 = 11
 * Result = 32 - 11 = 21
 *
 *
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
// discuss: https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let s = n.to_string();
        let mut sum = 0;
        let mut product = 1;
        for i in s.chars() {
            sum = sum + i as i32 - 48;
            if i as i32 == 0 {
                continue;
            }
            product = product * (i as i32 - 48);
        }
        product - sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1281_example_1() {
        let n = 234;
        let result = 15;

        assert_eq!(Solution::subtract_product_and_sum(n), result);
    }

    #[test]
    fn test_1281_example_2() {
        let n = 4421;
        let result = 21;

        assert_eq!(Solution::subtract_product_and_sum(n), result);
    }
}
