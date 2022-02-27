/**
 * [0542] 01 Matrix
 *
 * Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
 * The distance between two adjacent cells is 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: [[0,0,0],[0,1,0],[0,0,0]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-2-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
 * Output: [[0,0,0],[0,1,0],[1,2,1]]
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 10^4
 * 	1 <= m * n <= 10^4
 * 	mat[i][j] is either 0 or 1.
 * 	There is at least one 0 in mat.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/01-matrix/
// discuss: https://leetcode.com/problems/01-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = mat.len();
        if rows == 0 {
            return mat;
        }
        let cols = mat[0].len();
        let mut result = vec![vec![0; cols]; rows];

        // from left to right, up to down
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] != 0 {
                    let mut up = i32::MAX;
                    let mut left = std::i32::MAX;
                    if i > 0 {
                        up = result[i - 1][j];
                    }
                    if j > 0 {
                        left = result[i][j - 1];
                    }
                    let min = std::cmp::min(up, left);
                    if min == i32::MAX {
                        result[i][j] = min;
                    } else {
                        result[i][j] = min + 1;
                    }
                }
            }
        }

        // from right to left, down to up
        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                if mat[i][j] != 0 {
                    let mut down = i32::MAX;
                    let mut right = i32::MAX;
                    if i < rows - 1 {
                        down = result[i + 1][j];
                    }
                    if j < cols - 1 {
                        right = result[i][j + 1];
                    }
                    let min = std::cmp::min(down, right);
                    if min == i32::MAX {
                        result[i][j] = std::cmp::min(result[i][j], min);
                    } else {
                        result[i][j] = std::cmp::min(result[i][j], min + 1);
                    }
                }
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
    fn test_0542_example_1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let result = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];

        assert_eq!(Solution::update_matrix(mat), result);
    }

    #[test]
    fn test_0542_example_2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let result = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];

        assert_eq!(Solution::update_matrix(mat), result);
    }
}
