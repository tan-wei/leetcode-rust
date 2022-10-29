/**
 * [0867] Transpose Matrix
 *
 * Given a 2D integer array matrix, return the transpose of matrix.
 * The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/10/hint_transpose.png" style="width: 600px; height: 197px;" />
 *  
 * Example 1:
 *
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [[1,4,7],[2,5,8],[3,6,9]]
 *
 * Example 2:
 *
 * Input: matrix = [[1,2,3],[4,5,6]]
 * Output: [[1,4],[2,5],[3,6]]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 1000
 * 	1 <= m * n <= 10^5
 * 	-10^9 <= matrix[i][j] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/transpose-matrix/
// discuss: https://leetcode.com/problems/transpose-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for v in matrix.iter() {
            for (k2, v2) in v.iter().enumerate() {
                match result.get_mut(k2) {
                    Some(vec) => vec.push(*v2),
                    None => {
                        let mut vec: Vec<i32> = vec![*v2];
                        result.push(vec);
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
    fn test_0867_example_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

        assert_eq!(Solution::transpose(matrix), result);
    }

    #[test]
    fn test_0867_example_2() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

        assert_eq!(Solution::transpose(matrix), result);
    }
}
