/**
 * [0498] Diagonal Traverse
 *
 * Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/10/diag1-grid.jpg" style="width: 334px; height: 334px;" />
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,4,7,5,3,6,8,9]
 *
 * Example 2:
 *
 * Input: mat = [[1,2],[3,4]]
 * Output: [1,2,3,4]
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 10^4
 * 	1 <= m * n <= 10^4
 * 	-10^5 <= mat[i][j] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/diagonal-traverse/
// discuss: https://leetcode.com/problems/diagonal-traverse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if (mat.len() == 0 || mat[0].len() == 0) {
            return vec![];
        }

        let m = mat.len();
        let n = mat[0].len();

        let mut result = vec![];
        let mut i = 0;
        let mut j = 0;

        while result.len() < m * n {
            result.push(mat[i][j]);

            // move to next i and j
            if (i + j) % 2 == 0 {
                // moving up
                if j == n - 1 {
                    i += 1;
                } else if i == 0 {
                    j += 1;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else {
                // moving down
                if i == m - 1 {
                    j += 1;
                } else if j == 0 {
                    i += 1;
                } else {
                    i += 1;
                    j -= 1;
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
    fn test_0498_example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];

        assert_eq!(Solution::find_diagonal_order(mat), result);
    }

    #[test]
    fn test_0498_example_2() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let result = vec![1, 2, 3, 4];

        assert_eq!(Solution::find_diagonal_order(mat), result);
    }
}
