/**
 * [1582] Special Positions in a Binary Matrix
 *
 * Given an m x n binary matrix mat, return the number of special positions in mat.
 * A position (i, j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/special1.jpg" style="width: 244px; height: 245px;" />
 * Input: mat = [[1,0,0],[0,0,1],[1,0,0]]
 * Output: 1
 * Explanation: (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/special-grid.jpg" style="width: 244px; height: 245px;" />
 * Input: mat = [[1,0,0],[0,1,0],[0,0,1]]
 * Output: 3
 * Explanation: (0, 0), (1, 1) and (2, 2) are special positions.
 *
 *  
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	mat[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/special-positions-in-a-binary-matrix/
// discuss: https://leetcode.com/problems/special-positions-in-a-binary-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        mat.iter()
            .enumerate()
            .fold(
                (
                    0,
                    mat.iter().enumerate().fold(
                        (vec![0; mat.len()], vec![0; mat[0].len()]),
                        |(mut row, mut col), (i, v)| {
                            v.iter()
                                .enumerate()
                                .fold((row, col), |(mut row, mut col), (j, &x)| {
                                    if x == 1 {
                                        row[i] += 1;
                                        col[j] += 1;
                                    }
                                    (row, col)
                                })
                        },
                    ),
                ),
                |(mut c, (row, col)), (i, v)| {
                    v.iter().enumerate().for_each(|(j, &x)| {
                        if x == 1 && row[i] == 1 && col[j] == 1 {
                            c += 1;
                        }
                    });
                    (c, (row, col))
                },
            )
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1582_example_1() {
        let mat = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];

        let result = 1;

        assert_eq!(Solution::num_special(mat), result);
    }

    #[test]
    fn test_1582_example_2() {
        let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        let result = 3;

        assert_eq!(Solution::num_special(mat), result);
    }
}
