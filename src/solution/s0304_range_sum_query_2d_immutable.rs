/**
 * [304] Range Sum Query 2D - Immutable
 *
 * Given a 2D matrix matrix, handle multiple queries of the following type:
 *
 * 	Calculate the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 *
 * Implement the NumMatrix class:
 *
 * 	NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
 * 	int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/sum-grid.jpg" style="width: 415px; height: 415px;" />
 * Input
 * ["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
 * [[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
 * Output
 * [null, 8, 11, 12]
 * Explanation
 * NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
 * numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
 * numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
 * numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 200
 * 	-10^5 <= matrix[i][j] <= 10^5
 * 	0 <= row1 <= row2 < m
 * 	0 <= col1 <= col2 < n
 * 	At most 10^4 calls will be made to sumRegion.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-2d-immutable/
// discuss: https://leetcode.com/problems/range-sum-query-2d-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumMatrix(Vec<Vec<i32>>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut m = matrix.clone();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 {
                    m[i][j] += m[i - 1][j];
                }
                if j > 0 {
                    m[i][j] += m[i][j - 1];
                }
                if i > 0 && j > 0 {
                    m[i][j] -= m[i - 1][j - 1];
                }
            }
        }
        NumMatrix(m)
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut result = self.0[row2 as usize][col2 as usize];
        if row1 > 0 {
            result -= self.0[row1 as usize - 1][col2 as usize];
        }
        if col1 > 0 {
            result -= self.0[row2 as usize][col1 as usize - 1];
        }
        if row1 > 0 && col1 > 0 {
            result += self.0[row1 as usize - 1][col1 as usize - 1];
        }
        result
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_304() {}
}
