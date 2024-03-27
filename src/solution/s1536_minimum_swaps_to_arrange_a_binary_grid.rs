/**
 * [1536] Minimum Swaps to Arrange a Binary Grid
 *
 * Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.
 * A grid is said to be valid if all the cells above the main diagonal are zeros.
 * Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.
 * The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/28/fw.jpg" style="width: 750px; height: 141px;" />
 * Input: grid = [[0,0,1],[1,1,0],[1,0,0]]
 * Output: 3
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/16/e2.jpg" style="width: 270px; height: 270px;" />
 * Input: grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
 * Output: -1
 * Explanation: All rows are similar, swaps have no effect on the grid.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/16/e3.jpg" style="width: 200px; height: 200px;" />
 * Input: grid = [[1,0,0],[1,1,0],[1,1,1]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 200
 * 	grid[i][j] is either 0 or 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/
// discuss: https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        // Credit: https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/solutions/3167180/just-a-runnable-solution/
        let mut zeros = vec![0; grid.len()];
        for (i, row) in grid.iter().enumerate() {
            zeros[i] = row.iter().rev().take_while(|&&x| x == 0).count() as i32;
        }

        let mut swaps = 0;

        for i in 0..zeros.len() {
            if zeros[i] < (zeros.len() - i - 1) as i32 {
                let mut j = i + 1;
                while j < zeros.len() && zeros[j] < (zeros.len() - i - 1) as i32 {
                    j += 1;
                }
                if j == zeros.len() {
                    return -1;
                }
                while j > i {
                    zeros.swap(j, j - 1);
                    swaps += 1;
                    j -= 1;
                }
            }
        }

        swaps
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1536_example_1() {
        let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];

        let result = 3;

        assert_eq!(Solution::min_swaps(grid), result);
    }

    #[test]
    fn test_1536_example_2() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
        ];

        let result = -1;

        assert_eq!(Solution::min_swaps(grid), result);
    }

    #[test]
    fn test_1536_example_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];

        let result = 0;

        assert_eq!(Solution::min_swaps(grid), result);
    }
}
