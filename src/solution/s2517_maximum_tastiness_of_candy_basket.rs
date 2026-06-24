/**
 * [2517] Maximum Tastiness of Candy Basket
 *
 * You are given an array of positive integers price where price[i] denotes the price of the i^th candy and a positive integer k.
 * The store sells baskets of k distinct candies. The tastiness of a candy basket is the smallest absolute difference of the prices of any two candies in the basket.
 * Return the maximum tastiness of a candy basket.
 *  
 * Example 1:
 *
 * Input: price = [13,5,1,8,21,2], k = 3
 * Output: 8
 * Explanation: Choose the candies with the prices [13,5,21].
 * The tastiness of the candy basket is: min(|13 - 5|, |13 - 21|, |5 - 21|) = min(8, 8, 16) = 8.
 * It can be proven that 8 is the maximum tastiness that can be achieved.
 *
 * Example 2:
 *
 * Input: price = [1,3,1], k = 2
 * Output: 2
 * Explanation: Choose the candies with the prices [1,3].
 * The tastiness of the candy basket is: min(|1 - 3|) = min(2) = 2.
 * It can be proven that 2 is the maximum tastiness that can be achieved.
 *
 * Example 3:
 *
 * Input: price = [7,7,7,7], k = 2
 * Output: 0
 * Explanation: Choosing any two distinct candies from the candies we have will result in a tastiness of 0.
 *
 *  
 * Constraints:
 *
 * 	2 <= k <= price.length <= 10^5
 * 	1 <= price[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-tastiness-of-candy-basket/
// discuss: https://leetcode.com/problems/maximum-tastiness-of-candy-basket/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2517_example_1() {
        let price = vec![13, 5, 1, 8, 21, 2];
        let k = 3;

        let result = 8;

        assert_eq!(Solution::maximum_tastiness(price, k), result);
    }

    #[test]
    #[ignore]
    fn test_2517_example_2() {
        let price = vec![1, 3, 1];
        let k = 2;

        let result = 2;

        assert_eq!(Solution::maximum_tastiness(price, k), result);
    }

    #[test]
    #[ignore]
    fn test_2517_example_3() {
        let price = vec![7, 7, 7, 7];
        let k = 2;

        let result = 0;

        assert_eq!(Solution::maximum_tastiness(price, k), result);
    }
}
