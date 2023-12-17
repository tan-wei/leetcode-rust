/**
 * [1420] Build Array Where You Can Find The Maximum Exactly K Comparisons
 *
 * You are given three integers n, m and k. Consider the following algorithm to find the maximum element of an array of positive integers:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/e.png" style="width: 424px; height: 372px;" />
 * You should build the array arr which has the following properties:
 *
 * 	arr has exactly n integers.
 * 	1 <= arr[i] <= m where (0 <= i < n).
 * 	After applying the mentioned algorithm to arr, the value search_cost is equal to k.
 *
 * Return the number of ways to build the array arr under the mentioned conditions. As the answer may grow large, the answer must be computed modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 2, m = 3, k = 1
 * Output: 6
 * Explanation: The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
 *
 * Example 2:
 *
 * Input: n = 5, m = 2, k = 3
 * Output: 0
 * Explanation: There are no possible arrays that satisfy the mentioned conditions.
 *
 * Example 3:
 *
 * Input: n = 9, m = 1, k = 1
 * Output: 1
 * Explanation: The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 50
 * 	1 <= m <= 100
 * 	0 <= k <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// discuss: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/solutions/3111540/just-a-runnable-solution/
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let k = k as usize;
        if m < k {
            return 0;
        }
        let mut dp = vec![vec![vec![0; k + 1]; m + 1]; 2];
        let mod_num = 1_000_000_007;
        for j in 1..=m {
            dp[0][j][1] = j as i64;
        }
        for i in 1..n {
            for j in 1..=m {
                for l in 1..=std::cmp::min(i + 1, std::cmp::min(j, k)) {
                    dp[i & 1][j][l] = (dp[i & 1][j - 1][l]
                        + (dp[(i - 1) & 1][j][l] - dp[(i - 1) & 1][j - 1][l]) * j as i64
                        + dp[(i - 1) & 1][j - 1][l - 1])
                        % mod_num;
                }
            }
        }
        ((dp[(n - 1) & 1][m][k] + mod_num) % mod_num) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1420_example_1() {
        let n = 2;
        let m = 3;
        let k = 1;

        let result = 6;

        assert_eq!(Solution::num_of_arrays(n, m, k), result);
    }

    #[test]
    fn test_1420_example_2() {
        let n = 5;
        let m = 2;
        let k = 3;

        let result = 0;

        assert_eq!(Solution::num_of_arrays(n, m, k), result);
    }

    #[test]
    fn test_1420_example_3() {
        let n = 9;
        let m = 1;
        let k = 1;

        let result = 1;

        assert_eq!(Solution::num_of_arrays(n, m, k), result);
    }
}
