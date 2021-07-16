/**
 * [221] Maximal Square
 *
 * Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg" style="width: 400px; height: 319px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 4
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg" style="width: 165px; height: 165px;" />
 * Input: matrix = [["0","1"],["1","0"]]
 * Output: 1
 *
 * Example 3:
 *
 * Input: matrix = [["0"]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 300
 * 	matrix[i][j] is '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-square/
// discuss: https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let (m, n) = (matrix.len(), matrix[0].len());

        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut result = i32::MIN;

        for i in (0..=m - 1).rev() {
            for j in (0..=n - 1).rev() {
                if matrix[i][j] == '1' {
                    dp[i][j] =
                        std::cmp::min(std::cmp::min(dp[i + 1][j], dp[i][j + 1]), dp[i + 1][j + 1])
                            + 1;
                } else {
                    dp[i][j] = 0;
                }

                let edge = dp[i][j];
                result = std::cmp::max(edge * edge, result);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0221_example_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let result = 4;

        assert_eq!(Solution::maximal_square(matrix), result);
    }

    #[test]
    fn test_0221_example_2() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        let result = 1;

        assert_eq!(Solution::maximal_square(matrix), result);
    }

    #[test]
    fn test_0221_example_3() {
        let matrix = vec![vec!['0']];
        let result = 0;

        assert_eq!(Solution::maximal_square(matrix), result);
    }
}
