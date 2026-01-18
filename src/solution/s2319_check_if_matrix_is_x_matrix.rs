/**
 * [2319] Check if Matrix Is X-Matrix
 *
 * A square matrix is said to be an X-Matrix if both of the following conditions hold:
 * <ol>
 * 	All the elements in the diagonals of the matrix are non-zero.
 * 	All other elements are 0.
 * </ol>
 * Given a 2D integer array grid of size n x n representing a square matrix, return true if grid is an X-Matrix. Otherwise, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/03/ex1.jpg" style="width: 311px; height: 320px;" />
 * Input: grid = [[2,0,0,1],[0,3,1,0],[0,5,2,0],[4,0,0,2]]
 * Output: true
 * Explanation: Refer to the diagram above.
 * An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
 * Thus, grid is an X-Matrix.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/03/ex2.jpg" style="width: 238px; height: 246px;" />
 * Input: grid = [[5,7,0],[0,3,1],[0,5,0]]
 * Output: false
 * Explanation: Refer to the diagram above.
 * An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
 * Thus, grid is not an X-Matrix.
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	3 <= n <= 100
 * 	0 <= grid[i][j] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-matrix-is-x-matrix/
// discuss: https://leetcode.com/problems/check-if-matrix-is-x-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();

        for i in 0..n {
            for j in 0..n {
                if (grid[i][j] != 0) ^ ((j == i) || (i == n - 1 - j)) {
                    return false;
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2319_example_1() {
        let grid = vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];

        let result = true;

        assert_eq!(Solution::check_x_matrix(grid), result);
    }

    #[test]
    fn test_2319_example_2() {
        let grid = vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]];

        let result = false;

        assert_eq!(Solution::check_x_matrix(grid), result);
    }
}
