/**
 * [1716] Calculate Money in Leetcode Bank
 *
 * Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.
 * He starts by putting in $1 on Monday, the first day. Every day from Tuesday to Sunday, he will put in $1 more than the day before. On every subsequent Monday, he will put in $1 more than the previous Monday.<span style="display: none;"> </span>
 * Given n, return the total amount of money he will have in the Leetcode bank at the end of the n^th day.
 *  
 * Example 1:
 *
 * Input: n = 4
 * Output: 10
 * Explanation: After the 4^th day, the total is 1 + 2 + 3 + 4 = 10.
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 37
 * Explanation: After the 10^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2^nd Monday, Hercy only puts in $2.
 *
 * Example 3:
 *
 * Input: n = 20
 * Output: 96
 * Explanation: After the 20^th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/calculate-money-in-leetcode-bank/
// discuss: https://leetcode.com/problems/calculate-money-in-leetcode-bank/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut total: i32 = 0;

        let whole: i32 = n / 7;
        let remainder: i32 = n % 7;

        for i in 0..whole {
            total += 7 * i + 28;
        }

        for i in whole..whole + remainder {
            total += i + 1;
        }

        total
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1716_example_1() {
        let n = 4;

        let result = 10;

        assert_eq!(Solution::total_money(n), result);
    }

    #[test]
    fn test_1716_example_2() {
        let n = 10;

        let result = 37;

        assert_eq!(Solution::total_money(n), result);
    }

    #[test]
    fn test_1716_example_3() {
        let n = 20;

        let result = 96;

        assert_eq!(Solution::total_money(n), result);
    }
}
