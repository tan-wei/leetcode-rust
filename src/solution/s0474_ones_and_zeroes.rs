/**
 * [0474] Ones and Zeroes
 *
 * You are given an array of binary strings strs and two integers m and n.
 * Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
 * A set x is a subset of a set y if all elements of x are also elements of y.
 *  
 * Example 1:
 *
 * Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
 * Output: 4
 * Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
 * Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
 * {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
 *
 * Example 2:
 *
 * Input: strs = ["10","0","1"], m = 1, n = 1
 * Output: 2
 * Explanation: The largest subset is {"0", "1"}, so the answer is 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 600
 * 	1 <= strs[i].length <= 100
 * 	strs[i] consists only of digits '0' and '1'.
 * 	1 <= m, n <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ones-and-zeroes/
// discuss: https://leetcode.com/problems/ones-and-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

        for s in strs {
            let zeros = s.chars().filter(|&c| c == '0').count();
            let ones = s.chars().filter(|&c| c == '1').count();
            for i in (zeros..=(m as usize)).rev() {
                for j in (ones..=(n as usize)).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }

        dp[m as usize][n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0474_example_1() {
        let strs = vec_string!["10", "0001", "111001", "1", "0"];
        let m = 5;
        let n = 3;
        let result = 4;

        assert_eq!(Solution::find_max_form(strs, m, n), result);
    }

    #[test]
    fn test_0474_example_2() {
        let strs = vec_string!["10", "0", "1"];
        let m = 1;
        let n = 1;
        let result = 2;

        assert_eq!(Solution::find_max_form(strs, m, n), result);
    }
}
