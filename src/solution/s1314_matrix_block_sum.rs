/**
 * [1314] Matrix Block Sum
 *
 * Given a m x n matrix mat and an integer k, return a matrix answer where each answer[i][j] is the sum of all elements mat[r][c] for:
 *
 * 	i - k <= r <= i + k,
 * 	j - k <= c <= j + k, and
 * 	(r, c) is a valid position in the matrix.
 *
 *
 * Example 1:
 *
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
 * Output: [[12,21,16],[27,45,33],[24,39,28]]
 *
 * Example 2:
 *
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
 * Output: [[45,45,45],[45,45,45],[45,45,45]]
 *
 *
 * Constraints:
 *
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n, k <= 100
 * 	1 <= mat[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/matrix-block-sum/
// discuss: https://leetcode.com/problems/matrix-block-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let k = k as usize;
        let mut memo = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                memo[i + 1][j + 1] = memo[i + 1][j] + memo[i][j + 1] - memo[i][j] + mat[i][j];
            }
        }

        let mut result = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                let i_min = i.saturating_sub(k);
                let i_max = std::cmp::min(i + k, n - 1);
                let j_min = j.saturating_sub(k);
                let j_max = std::cmp::min(j + k, m - 1);

                result[i][j] =
                    memo[i_max + 1][j_max + 1] - memo[i_max + 1][j_min] - memo[i_min][j_max + 1]
                        + memo[i_min][j_min];
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
    fn test_1314_example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let result = vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]];

        assert_eq!(Solution::matrix_block_sum(mat, k), result);
    }

    #[test]
    fn test_1314_example_2() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 2;
        let result = vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]];

        assert_eq!(Solution::matrix_block_sum(mat, k), result);
    }
}
