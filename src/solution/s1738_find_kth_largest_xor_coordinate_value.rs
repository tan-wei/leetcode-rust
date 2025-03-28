/**
 * [1738] Find Kth Largest XOR Coordinate Value
 *
 * You are given a 2D matrix of size m x n, consisting of non-negative integers. You are also given an integer k.
 * The value of coordinate (a, b) of the matrix is the XOR of all matrix[i][j] where 0 <= i <= a < m and 0 <= j <= b < n (0-indexed).
 * Find the k^th largest value (1-indexed) of all the coordinates of matrix.
 *  
 * Example 1:
 *
 * Input: matrix = [[5,2],[1,6]], k = 1
 * Output: 7
 * Explanation: The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
 *
 * Example 2:
 *
 * Input: matrix = [[5,2],[1,6]], k = 2
 * Output: 5
 * Explanation: The value of coordinate (0,0) is 5 = 5, which is the 2nd largest value.
 *
 * Example 3:
 *
 * Input: matrix = [[5,2],[1,6]], k = 3
 * Output: 4
 * Explanation: The value of coordinate (1,0) is 5 XOR 1 = 4, which is the 3rd largest value.
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 1000
 * 	0 <= matrix[i][j] <= 10^6
 * 	1 <= k <= m * n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-kth-largest-xor-coordinate-value/
// discuss: https://leetcode.com/problems/find-kth-largest-xor-coordinate-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut result = vec![];

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j] ^ dp[i][j - 1] ^ dp[i - 1][j - 1] ^ matrix[i - 1][j - 1];
                result.push(dp[i][j]);
            }
        }

        result.sort_unstable();
        result[result.len() - k as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1738_example_1() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 1;

        let result = 7;

        assert_eq!(Solution::kth_largest_value(matrix, k), result);
    }

    #[test]
    fn test_1738_example_2() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 2;

        let result = 5;

        assert_eq!(Solution::kth_largest_value(matrix, k), result);
    }

    #[test]
    fn test_1738_example_3() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 3;

        let result = 4;

        assert_eq!(Solution::kth_largest_value(matrix, k), result);
    }
}
