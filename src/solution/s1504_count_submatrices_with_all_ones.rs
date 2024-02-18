/**
 * [1504] Count Submatrices With All Ones
 *
 * Given an m x n binary matrix mat, return the number of submatrices that have all ones.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/27/ones1-grid.jpg" style="width: 244px; height: 245px;" />
 * Input: mat = [[1,0,1],[1,1,0],[1,1,0]]
 * Output: 13
 * Explanation:
 * There are 6 rectangles of side 1x1.
 * There are 2 rectangles of side 1x2.
 * There are 3 rectangles of side 2x1.
 * There is 1 rectangle of side 2x2.
 * There is 1 rectangle of side 3x1.
 * Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/27/ones2-grid.jpg" style="width: 324px; height: 245px;" />
 * Input: mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
 * Output: 24
 * Explanation:
 * There are 8 rectangles of side 1x1.
 * There are 5 rectangles of side 1x2.
 * There are 2 rectangles of side 1x3.
 * There are 4 rectangles of side 2x1.
 * There are 2 rectangles of side 2x2.
 * There are 2 rectangles of side 3x1.
 * There is 1 rectangle of side 3x2.
 * Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 150
 * 	mat[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-submatrices-with-all-ones/
// discuss: https://leetcode.com/problems/count-submatrices-with-all-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let m = mat.len();
        let n = mat[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    dp[i][j] = if j == 0 { 1 } else { dp[i][j - 1] + 1 };
                    let mut width = dp[i][j];
                    for k in (0..=i).rev() {
                        width = width.min(dp[k][j]);
                        result += width;
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
    fn test_1504_example_1() {
        let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];

        let result = 13;

        assert_eq!(Solution::num_submat(mat), result);
    }

    #[test]
    fn test_1504_example_2() {
        let mat = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];

        let result = 24;

        assert_eq!(Solution::num_submat(mat), result);
    }
}
