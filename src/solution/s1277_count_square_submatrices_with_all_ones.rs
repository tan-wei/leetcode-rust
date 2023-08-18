/**
 * [1277] Count Square Submatrices with All Ones
 *
 * Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
 *
 * Example 1:
 *
 * Input: matrix =
 * [
 *   [0,1,1,1],
 *   [1,1,1,1],
 *   [0,1,1,1]
 * ]
 * Output: 15
 * Explanation:
 * There are 10 squares of side 1.
 * There are 4 squares of side 2.
 * There is  1 square of side 3.
 * Total number of squares = 10 + 4 + 1 = 15.
 *
 * Example 2:
 *
 * Input: matrix =
 * [
 *   [1,0,1],
 *   [1,1,0],
 *   [1,1,0]
 * ]
 * Output: 7
 * Explanation:
 * There are 6 squares of side 1.
 * There is 1 square of side 2.
 * Total number of squares = 6 + 1 = 7.
 *
 *
 * Constraints:
 *
 * 	1 <= arr.length <= 300
 * 	1 <= arr[0].length <= 300
 * 	0 <= arr[i][j] <= 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-square-submatrices-with-all-ones/
// discuss: https://leetcode.com/problems/count-square-submatrices-with-all-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut matrix = matrix;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 && j > 0 && matrix[i][j] > 0 {
                    matrix[i][j] = 1 + std::cmp::min(
                        std::cmp::min(matrix[i - 1][j], matrix[i - 1][j - 1]),
                        matrix[i][j - 1],
                    );
                }
                count += matrix[i][j];
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1277_example_1() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        let result = 15;

        assert_eq!(Solution::count_squares(matrix), result);
    }

    #[test]
    fn test_1277_example_2() {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let result = 7;

        assert_eq!(Solution::count_squares(matrix), result);
    }
}
