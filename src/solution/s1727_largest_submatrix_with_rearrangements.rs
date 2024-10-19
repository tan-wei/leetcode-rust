/**
 * [1727] Largest Submatrix With Rearrangements
 *
 * You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of the matrix in any order.
 * Return the area of the largest submatrix within matrix where every element of the submatrix is 1 after reordering the columns optimally.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40536-pm.png" style="width: 500px; height: 240px;" />
 * Input: matrix = [[0,0,1],[1,1,1],[1,0,1]]
 * Output: 4
 * Explanation: You can rearrange the columns as shown above.
 * The largest submatrix of 1s, in bold, has an area of 4.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40852-pm.png" style="width: 500px; height: 62px;" />
 * Input: matrix = [[1,0,1,0,1]]
 * Output: 3
 * Explanation: You can rearrange the columns as shown above.
 * The largest submatrix of 1s, in bold, has an area of 3.
 *
 * Example 3:
 *
 * Input: matrix = [[1,1,0],[1,0,1]]
 * Output: 2
 * Explanation: Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m * n <= 10^5
 * 	matrix[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-submatrix-with-rearrangements/
// discuss: https://leetcode.com/problems/largest-submatrix-with-rearrangements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let (n, m, mut result) = (matrix.len(), matrix[0].len(), 0);

        for i in 1..n {
            for j in 0..m {
                if (matrix[i][j] != 0) {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }

        for i in 0..n {
            matrix[i].sort_unstable();

            for j in 0..m {
                result = result.max((m - j) as i32 * matrix[i][j]);
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
    fn test_1727_example_1() {
        let matrix = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];

        let result = 4;

        assert_eq!(Solution::largest_submatrix(matrix), result);
    }

    #[test]
    fn test_1727_example_2() {
        let matrix = vec![vec![1, 0, 1, 0, 1]];

        let result = 3;

        assert_eq!(Solution::largest_submatrix(matrix), result);
    }

    #[test]
    fn test_1727_example_3() {
        let matrix = vec![vec![1, 1, 0], vec![1, 0, 1]];

        let result = 2;

        assert_eq!(Solution::largest_submatrix(matrix), result);
    }
}
