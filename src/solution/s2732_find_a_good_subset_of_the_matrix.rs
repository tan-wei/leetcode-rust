/**
 * [2732] Find a Good Subset of the Matrix
 *
 * You are given a 0-indexed m x n binary matrix grid.
 * Let us call a non-empty subset of rows good if the sum of each column of the subset is at most half of the length of the subset.
 * More formally, if the length of the chosen subset of rows is k, then the sum of each column should be at most floor(k / 2).
 * Return an integer array that contains row indices of a good subset sorted in ascending order.
 * If there are multiple good subsets, you can return any of them. If there are no good subsets, return an empty array.
 * A subset of rows of the matrix grid is any matrix that can be obtained by deleting some (possibly none or all) rows from grid.
 *  
 * Example 1:
 *
 * Input: grid = [[0,1,1,0],[0,0,0,1],[1,1,1,1]]
 * Output: [0,1]
 * Explanation: We can choose the 0^th and 1^st rows to create a good subset of rows.
 * The length of the chosen subset is 2.
 * - The sum of the 0^th column is 0 + 0 = 0, which is at most half of the length of the subset.
 * - The sum of the 1^st column is 1 + 0 = 1, which is at most half of the length of the subset.
 * - The sum of the 2^nd column is 1 + 0 = 1, which is at most half of the length of the subset.
 * - The sum of the 3^rd column is 0 + 1 = 1, which is at most half of the length of the subset.
 *
 * Example 2:
 *
 * Input: grid = [[0]]
 * Output: [0]
 * Explanation: We can choose the 0^th row to create a good subset of rows.
 * The length of the chosen subset is 1.
 * - The sum of the 0^th column is 0, which is at most half of the length of the subset.
 *
 * Example 3:
 *
 * Input: grid = [[1,1,1],[1,1,1]]
 * Output: []
 * Explanation: It is impossible to choose any subset of rows to create a good subset.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m <= 10^4
 * 	1 <= n <= 5
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-a-good-subset-of-the-matrix/
// discuss: https://leetcode.com/problems/find-a-good-subset-of-the-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2732_example_1() {
        let grid = vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]];

        let result = vec![0, 1];

        assert_eq!(Solution::good_subsetof_binary_matrix(grid), result);
    }

    #[test]
    #[ignore]
    fn test_2732_example_2() {
        let grid = vec![vec![0]];

        let result = vec![0];

        assert_eq!(Solution::good_subsetof_binary_matrix(grid), result);
    }

    #[test]
    #[ignore]
    fn test_2732_example_3() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];

        let result: Vec<i32> = vec![];

        assert_eq!(Solution::good_subsetof_binary_matrix(grid), result);
    }
}
