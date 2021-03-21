/**
 * [73] Set Matrix Zeroes
 *
 * Given an m x n matrix. If an element is 0, set its entire row and column to 0. Do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>.
 * Follow up:
 *
 * 	A straight forward solution using O(mn) space is probably a bad idea.
 * 	A simple improvement uses O(m + n) space, but still not the best solution.
 * 	Could you devise a constant space solution?
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat1.jpg" style="width: 450px; height: 169px;" />
 * Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[1,0,1],[0,0,0],[1,0,1]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat2.jpg" style="width: 450px; height: 137px;" />
 * Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[0].length
 * 	1 <= m, n <= 200
 * 	-2^31 <= matrix[i][j] <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-matrix-zeroes/
// discuss: https://leetcode.com/problems/set-matrix-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    const M: usize = 200;
    const N: usize = 200;
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut is_zero_col = [false; Self::M];
        let mut is_zero_row = [false; Self::N];

        for (row, inner) in matrix.iter().enumerate() {
            for (col, &v) in inner.iter().enumerate() {
                if v == 0 {
                    is_zero_col[col] = true;
                    is_zero_row[row] = true;
                }
            }
        }

        for (row, inner) in matrix.iter_mut().enumerate() {
            for (col, v) in inner.iter_mut().enumerate() {
                if is_zero_col[col] || is_zero_row[row] {
                    *v = 0;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0073_example_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let result = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, result);
    }

    #[test]
    fn test_0073_example_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let result = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, result);
    }
}
