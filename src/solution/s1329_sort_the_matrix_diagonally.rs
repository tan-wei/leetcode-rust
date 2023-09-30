/**
 * [1329] Sort the Matrix Diagonally
 *
 * A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row or leftmost column and going in the bottom-right direction until reaching the matrix's end. For example, the matrix diagonal starting from mat[2][0], where mat is a 6 x 3 matrix, includes cells mat[2][0], mat[3][1], and mat[4][2].
 * Given an m x n matrix mat of integers, sort each matrix diagonal in ascending order and return the resulting matrix.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/1482_example_1_2.png" style="width: 500px; height: 198px;" />
 * Input: mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
 * Output: [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
 *
 * Example 2:
 *
 * Input: mat = [[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]
 * Output: [[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
 *
 *
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	1 <= mat[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-matrix-diagonally/
// discuss: https://leetcode.com/problems/sort-the-matrix-diagonally/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let n = mat.len();
        let m = mat[0].len();

        for i in (0..m).rev() {
            let mut r = 0;
            let mut c = i;
            let mut temp = vec![];
            while r < n && c < m {
                temp.push(mat[r][c]);
                r += 1;
                c += 1;
            }

            temp.sort();
            let mut r = 0;
            let mut c = i;
            let mut ci = 0;
            while r < n && c < m {
                mat[r][c] = temp[ci];
                ci += 1;
                r += 1;
                c += 1;
            }
        }

        for i in 1..n {
            let mut r = i;
            let mut c = 0;
            let mut temp = vec![];
            while r < n && c < m {
                temp.push(mat[r][c]);
                r += 1;
                c += 1;
            }

            temp.sort();
            let mut r = i;
            let mut c = 0;
            let mut ci = 0;
            while r < n && c < m {
                mat[r][c] = temp[ci];
                ci += 1;
                r += 1;
                c += 1;
            }
        }

        mat
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1329_example_1() {
        let mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
        let result = vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]];

        assert_eq!(Solution::diagonal_sort(mat), result);
    }

    #[test]
    fn test_1329_example_2() {
        let mat = vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50],
        ];
        let result = vec![
            vec![5, 17, 4, 1, 52, 7],
            vec![11, 11, 25, 45, 8, 69],
            vec![14, 23, 25, 44, 58, 15],
            vec![22, 27, 31, 36, 50, 66],
            vec![84, 28, 75, 33, 55, 68],
        ];

        assert_eq!(Solution::diagonal_sort(mat), result);
    }
}
