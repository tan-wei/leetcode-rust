/**
 * [0879] Profitable Schemes
 *
 * There is a group of n members, and a list of various crimes they could commit. The i^th crime generates a profit[i] and requires group[i] members to participate in it. If a member participates in one crime, that member can't participate in another crime.
 * Let's call a profitable scheme any subset of these crimes that generates at least minProfit profit, and the total number of members participating in that subset of crimes is at most n.
 * Return the number of schemes that can be chosen. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 5, minProfit = 3, group = [2,2], profit = [2,3]
 * Output: 2
 * Explanation: To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
 * In total, there are 2 schemes.
 * Example 2:
 *
 * Input: n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
 * Output: 7
 * Explanation: To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
 * There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	0 <= minProfit <= 100
 * 	1 <= group.length <= 100
 * 	1 <= group[i] <= 100
 * 	profit.length == group.length
 * 	0 <= profit[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/profitable-schemes/
// discuss: https://leetcode.com/problems/profitable-schemes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/profitable-schemes/solutions/828340/rust-translated-8ms-100/
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; min_profit as usize + 1];
        dp[0][0] = 1;
        let mut result = 0;
        for k in 0..group.len() {
            let g1 = group[k];
            let p1 = profit[k];
            for i in (0..min_profit + 1).rev() {
                for j in (0..=n - g1).rev() {
                    dp[std::cmp::min(i + p1, min_profit) as usize][(j + g1) as usize] = (dp
                        [std::cmp::min(i + p1, min_profit) as usize][(j + g1) as usize]
                        + dp[i as usize][j as usize])
                        % MOD;
                }
            }
        }
        for &x in &dp[min_profit as usize] {
            result = (result + x) % MOD;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0879_example_1() {
        let n = 5;
        let min_profit = 3;
        let group = vec![2, 2];
        let profit = vec![2, 3];
        let result = 2;

        assert_eq!(
            Solution::profitable_schemes(n, min_profit, group, profit),
            result
        );
    }

    #[test]
    fn test_0879_example_2() {
        let n = 10;
        let min_profit = 5;
        let group = vec![2, 3, 5];
        let profit = vec![6, 7, 8];
        let result = 7;

        assert_eq!(
            Solution::profitable_schemes(n, min_profit, group, profit),
            result
        );
    }
}
