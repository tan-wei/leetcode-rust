/**
 * [2482] Difference Between Ones and Zeros in Row and Column
 *
 * You are given a 0-indexed m x n binary matrix grid.
 * A 0-indexed m x n difference matrix diff is created with the following procedure:
 *
 * 	Let the number of ones in the i^th row be onesRowi.
 * 	Let the number of ones in the j^th column be onesColj.
 * 	Let the number of zeros in the i^th row be zerosRowi.
 * 	Let the number of zeros in the j^th column be zerosColj.
 * 	diff[i][j] = onesRowi + onesColj - zerosRowi - zerosColj
 *
 * Return the difference matrix diff.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2022/11/06/image-20221106171729-5.png" style="width: 400px; height: 208px;" />
 * Input: grid = [[0,1,1],[1,0,1],[0,0,1]]
 * Output: [[0,0,4],[0,0,4],[-2,-2,2]]
 * Explanation:
 * - diff[0][0] = onesRow0 + onesCol0 - zerosRow0 - zerosCol0 = 2 + 1 - 1 - 2 = 0
 * - diff[0][1] = onesRow0 + onesCol1 - zerosRow0 - zerosCol1 = 2 + 1 - 1 - 2 = 0
 * - diff[0][2] = onesRow0 + onesCol2 - zerosRow0 - zerosCol2 = 2 + 3 - 1 - 0 = 4
 * - diff[1][0] = onesRow1 + onesCol0 - zerosRow1 - zerosCol0 = 2 + 1 - 1 - 2 = 0
 * - diff[1][1] = onesRow1 + onesCol1 - zerosRow1 - zerosCol1 = 2 + 1 - 1 - 2 = 0
 * - diff[1][2] = onesRow1 + onesCol2 - zerosRow1 - zerosCol2 = 2 + 3 - 1 - 0 = 4
 * - diff[2][0] = onesRow2 + onesCol0 - zerosRow2 - zerosCol0 = 1 + 1 - 2 - 2 = -2
 * - diff[2][1] = onesRow2 + onesCol1 - zerosRow2 - zerosCol1 = 1 + 1 - 2 - 2 = -2
 * - diff[2][2] = onesRow2 + onesCol2 - zerosRow2 - zerosCol2 = 1 + 3 - 2 - 0 = 2
 *
 * Example 2:
 * <img src="https://assets.leetcode.com/uploads/2022/11/06/image-20221106171747-6.png" style="width: 358px; height: 150px;" />
 * Input: grid = [[1,1,1],[1,1,1]]
 * Output: [[5,5,5],[5,5,5]]
 * Explanation:
 * - diff[0][0] = onesRow0 + onesCol0 - zerosRow0 - zerosCol0 = 3 + 2 - 0 - 0 = 5
 * - diff[0][1] = onesRow0 + onesCol1 - zerosRow0 - zerosCol1 = 3 + 2 - 0 - 0 = 5
 * - diff[0][2] = onesRow0 + onesCol2 - zerosRow0 - zerosCol2 = 3 + 2 - 0 - 0 = 5
 * - diff[1][0] = onesRow1 + onesCol0 - zerosRow1 - zerosCol0 = 3 + 2 - 0 - 0 = 5
 * - diff[1][1] = onesRow1 + onesCol1 - zerosRow1 - zerosCol1 = 3 + 2 - 0 - 0 = 5
 * - diff[1][2] = onesRow1 + onesCol2 - zerosRow1 - zerosCol2 = 3 + 2 - 0 - 0 = 5
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 10^5
 * 	1 <= m * n <= 10^5
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
// discuss: https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/solutions/2937809/optimized-rust-solution-by-tylerbloom-h7bj/
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows_ones: Vec<i32> = grid.iter().map(|r| r.iter().sum()).collect();
        let mut grid = grid;
        let row_len = grid.len() as i32;

        let cols_ones: Vec<i32> = (0..grid[0].len())
            .map(|i| grid.iter().map(|r| r[i]).sum())
            .collect();
        let col_len = grid[0].len() as i32;
        let grid_val = row_len + col_len;

        grid.iter_mut().enumerate().for_each(|(i, r)| {
            r.iter_mut()
                .enumerate()
                .for_each(|(j, v)| *v = 2 * (rows_ones[i] + cols_ones[j]) - grid_val)
        });
        grid
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2482_example_1() {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];

        let result = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];

        assert_eq!(Solution::ones_minus_zeros(grid), result);
    }

    #[test]
    fn test_2482_example_2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];

        let result = vec![vec![5, 5, 5], vec![5, 5, 5]];

        assert_eq!(Solution::ones_minus_zeros(grid), result);
    }
}
