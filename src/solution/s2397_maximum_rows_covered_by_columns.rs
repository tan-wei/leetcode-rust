/**
 * [2397] Maximum Rows Covered by Columns
 *
 * You are given an m x n binary matrix matrix and an integer numSelect.
 * Your goal is to select exactly numSelect distinct columns from matrix such that you cover as many rows as possible.
 * A row is considered covered if all the 1's in that row are also part of a column that you have selected. If a row does not have any 1s, it is also considered covered.
 * More formally, let us consider selected = {c1, c2, ...., cnumSelect} as the set of columns selected by you. A row i is covered by selected if:
 *
 * 	For each cell where matrix[i][j] == 1, the column j is in selected.
 * 	Or, no cell in row i has a value of 1.
 *
 * Return the maximum number of rows that can be covered by a set of numSelect columns.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/14/rowscovered.png" style="width: 240px; height: 400px;" />
 * <div class="example-block">
 * Input: <span class="example-io">matrix = [[0,0,0],[1,0,1],[0,1,1],[0,0,1]], numSelect = 2</span>
 * Output: <span class="example-io">3</span>
 * Explanation:
 * One possible way to cover 3 rows is shown in the diagram above.<br />
 * We choose s = {0, 2}.<br />
 * - Row 0 is covered because it has no occurrences of 1.<br />
 * - Row 1 is covered because the columns with value 1, i.e. 0 and 2 are present in s.<br />
 * - Row 2 is not covered because matrix[2][1] == 1 but 1 is not present in s.<br />
 * - Row 3 is covered because matrix[2][2] == 1 and 2 is present in s.<br />
 * Thus, we can cover three rows.<br />
 * Note that s = {1, 2} will also cover 3 rows, but it can be shown that no more than three rows can be covered.
 * </div>
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/14/rowscovered2.png" style="height: 250px; width: 84px;" />
 * <div class="example-block">
 * Input: <span class="example-io">matrix = [[1],[0]], numSelect = 1</span>
 * Output: <span class="example-io">2</span>
 * Explanation:
 * Selecting the only column will result in both rows being covered since the entire matrix is selected.
 * </div>
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 12
 * 	matrix[i][j] is either 0 or 1.
 * 	1 <= numSelect <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-rows-covered-by-columns/
// discuss: https://leetcode.com/problems/maximum-rows-covered-by-columns/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_2397_example_1() {
        let matrix = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]];
        let num_select = 2;

        let result = 3;

        assert_eq!(Solution::maximum_rows(matrix, num_select), result);
    }

    #[test]
    #[ignore]
    fn test_2397_example_2() {
        let matrix = vec![vec![1], vec![0]];
        let num_select = 1;

        let result = 2;

        assert_eq!(Solution::maximum_rows(matrix, num_select), result);
    }
}
