/**
 * [0566] Reshape the Matrix
 *
 * In MATLAB, there is a handy function called reshape which can reshape an m x n matrix into a new one with a different size r x c keeping its original data.
 * You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
 * The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
 * If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/reshape1-grid.jpg" style="width: 613px; height: 173px;" />
 * Input: mat = [[1,2],[3,4]], r = 1, c = 4
 * Output: [[1,2,3,4]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/reshape2-grid.jpg" style="width: 453px; height: 173px;" />
 * Input: mat = [[1,2],[3,4]], r = 2, c = 4
 * Output: [[1,2],[3,4]]
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	-1000 <= mat[i][j] <= 1000
 * 	1 <= r, c <= 300
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reshape-the-matrix/
// discuss: https://leetcode.com/problems/reshape-the-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat;
        }

        let mut result = vec![vec![0; c as usize]; r as usize];
        let mut values = mat.iter().flat_map(|row| row.iter());
        for col in result.iter_mut().flat_map(|row| row.iter_mut()) {
            if let Some(&val) = values.next() {
                *col = val;
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
    fn test_0566_example_1() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let r = 1;
        let c = 4;
        let result = vec![vec![1, 2, 3, 4]];

        assert_eq!(Solution::matrix_reshape(mat, r, c), result);
    }
}
