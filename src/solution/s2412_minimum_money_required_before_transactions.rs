/**
 * [2412] Minimum Money Required Before Transactions
 *
 * You are given a 0-indexed 2D integer array <font face="monospace">transactions</font>, where transactions[i] = [costi, cashbacki].
 * The array describes transactions, where each transaction must be completed exactly once in some order. At any given moment, you have a certain amount of money. In order to complete transaction i, money >= costi must hold true. After performing a transaction, money becomes money - costi + cashbacki.
 * Return the minimum amount of money required before any transaction so that all of the transactions can be completed regardless of the order of the transactions.
 *  
 * Example 1:
 *
 * Input: transactions = [[2,1],[5,0],[4,2]]
 * Output: 10
 * Explanation:
 * Starting with money = 10, the transactions can be performed in any order.
 * It can be shown that starting with money < 10 will fail to complete all transactions in some order.
 *
 * Example 2:
 *
 * Input: transactions = [[3,0],[0,3]]
 * Output: 3
 * Explanation:
 * - If transactions are in the order [[3,0],[0,3]], the minimum money required to complete the transactions is 3.
 * - If transactions are in the order [[0,3],[3,0]], the minimum money required to complete the transactions is 0.
 * Thus, starting with money = 3, the transactions can be performed in any order.
 *
 *  
 * Constraints:
 *
 * 	1 <= transactions.length <= 10^5
 * 	transactions[i].length == 2
 * 	0 <= costi, cashbacki <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-money-required-before-transactions/
// discuss: https://leetcode.com/problems/minimum-money-required-before-transactions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2412_example_1() {
        let transactions = vec![vec![2, 1], vec![5, 0], vec![4, 2]];

        let result = 10;

        assert_eq!(Solution::minimum_money(transactions), result);
    }

    #[test]
    #[ignore]
    fn test_2412_example_2() {
        let transactions = vec![vec![2, 1], vec![5, 0], vec![4, 2]];

        let result = 10;

        assert_eq!(Solution::minimum_money(transactions), result);
    }
}
