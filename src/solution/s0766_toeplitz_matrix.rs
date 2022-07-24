/**
 * [0766] Toeplitz Matrix
 *
 * Given an m x n matrix, return true if the matrix is Toeplitz. Otherwise, return false.
 * A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same elements.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/ex1.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
 * Output: true
 * Explanation:
 * In the above grid, the diagonals are:
 * "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
 * In each diagonal all elements are the same, so the answer is True.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/ex2.jpg" style="width: 162px; height: 162px;" />
 * Input: matrix = [[1,2],[2,2]]
 * Output: false
 * Explanation:
 * The diagonal "[1, 2]" has different elements.
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 20
 * 	0 <= matrix[i][j] <= 99
 *
 *  
 * Follow up:
 *
 * 	What if the matrix is stored on disk, and the memory is limited such that you can only load at most one row of the matrix into the memory at once?
 * 	What if the matrix is so large that you can only load up a partial row into the memory at once?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/toeplitz-matrix/
// discuss: https://leetcode.com/problems/toeplitz-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        if m == 0 {
            unreachable!();
        }
        let n = matrix[0].len();
        if n == 0 {
            unreachable!();
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] != matrix[i - 1][j - 1] {
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
    fn test_0766_example_1() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        let result = true;

        assert_eq!(Solution::is_toeplitz_matrix(matrix), result);
    }

    #[test]
    fn test_0766_example_2() {
        let matrix = vec![vec![1, 2], vec![2, 2]];
        let result = false;

        assert_eq!(Solution::is_toeplitz_matrix(matrix), result);
    }
}
