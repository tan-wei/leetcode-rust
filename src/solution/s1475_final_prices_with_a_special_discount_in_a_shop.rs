/**
 * [1475] Final Prices With a Special Discount in a Shop
 *
 * You are given an integer array prices where prices[i] is the price of the i^th item in a shop.
 * There is a special discount for items in the shop. If you buy the i^th item, then you will receive a discount equivalent to prices[j] where j is the minimum index such that j > i and prices[j] <= prices[i]. Otherwise, you will not receive any discount at all.
 * Return an integer array answer where answer[i] is the final price you will pay for the i^th item of the shop, considering the special discount.
 *  
 * Example 1:
 *
 * Input: prices = [8,4,6,2,3]
 * Output: [4,2,4,2,3]
 * Explanation:
 * For item 0 with price[0]=8 you will receive a discount equivalent to prices[1]=4, therefore, the final price you will pay is 8 - 4 = 4.
 * For item 1 with price[1]=4 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 4 - 2 = 2.
 * For item 2 with price[2]=6 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 6 - 2 = 4.
 * For items 3 and 4 you will not receive any discount at all.
 *
 * Example 2:
 *
 * Input: prices = [1,2,3,4,5]
 * Output: [1,2,3,4,5]
 * Explanation: In this case, for all items, you will not receive any discount at all.
 *
 * Example 3:
 *
 * Input: prices = [10,1,1,6]
 * Output: [9,0,1,6]
 *
 *  
 * Constraints:
 *
 * 	1 <= prices.length <= 500
 * 	1 <= prices[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
// discuss: https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = prices;
        let l = result.len();
        let mut stack = Vec::with_capacity(l);

        for i in 0..l {
            while !stack.is_empty() && result[*stack.last().unwrap()] >= result[i] {
                result[stack.pop().unwrap()] -= result[i];
            }
            stack.push(i);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1475_example_1() {
        let prices = vec![8, 4, 6, 2, 3];

        let result = vec![4, 2, 4, 2, 3];

        assert_eq!(Solution::final_prices(prices), result);
    }

    #[test]
    fn test_1475_example_2() {
        let prices = vec![1, 2, 3, 4, 5];

        let result = vec![1, 2, 3, 4, 5];

        assert_eq!(Solution::final_prices(prices), result);
    }

    #[test]
    fn test_1475_example_3() {
        let prices = vec![10, 1, 1, 6];

        let result = vec![9, 0, 1, 6];

        assert_eq!(Solution::final_prices(prices), result);
    }
}
