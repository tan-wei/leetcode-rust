/**
 * [2240] Number of Ways to Buy Pens and Pencils
 *
 * You are given an integer total indicating the amount of money you have. You are also given two integers cost1 and cost2 indicating the price of a pen and pencil respectively. You can spend part or all of your money to buy multiple quantities (or none) of each kind of writing utensil.
 * Return the number of distinct ways you can buy some number of pens and pencils.
 *  
 * Example 1:
 *
 * Input: total = 20, cost1 = 10, cost2 = 5
 * Output: 9
 * Explanation: The price of a pen is 10 and the price of a pencil is 5.
 * - If you buy 0 pens, you can buy 0, 1, 2, 3, or 4 pencils.
 * - If you buy 1 pen, you can buy 0, 1, or 2 pencils.
 * - If you buy 2 pens, you cannot buy any pencils.
 * The total number of ways to buy pens and pencils is 5 + 3 + 1 = 9.
 *
 * Example 2:
 *
 * Input: total = 5, cost1 = 10, cost2 = 10
 * Output: 1
 * Explanation: The price of both pens and pencils are 10, which cost more than total, so you cannot buy any writing utensils. Therefore, there is only 1 way: buy 0 pens and 0 pencils.
 *
 *  
 * Constraints:
 *
 * 	1 <= total, cost1, cost2 <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-buy-pens-and-pencils/
// discuss: https://leetcode.com/problems/number-of-ways-to-buy-pens-and-pencils/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2240_example_1() {
        let total = 20;
        let cost1 = 10;
        let cost2 = 5;

        let result = 9;

        assert_eq!(
            Solution::ways_to_buy_pens_pencils(total, cost1, cost2),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2240_example_2() {
        let total = 5;
        let cost1 = 10;
        let cost2 = 10;

        let result = 1;

        assert_eq!(
            Solution::ways_to_buy_pens_pencils(total, cost1, cost2),
            result
        );
    }
}
