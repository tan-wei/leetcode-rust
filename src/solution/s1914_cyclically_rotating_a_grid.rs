/**
 * [1914] Cyclically Rotating a Grid
 *
 * You are given an m x n integer matrix grid​​​, where m and n are both even integers, and an integer k.
 *
 * The matrix is composed of several layers, which is shown in the below image, where each color is its own layer:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/10/ringofgrid.png" style="width: 231px; height: 258px;" />
 *
 * A cyclic rotation of the matrix is done by cyclically rotating each layer in the matrix. To cyclically rotate a layer once, each element in the layer will take the place of the adjacent element in the counter-clockwise direction. An example rotation is shown below:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/22/explanation_grid.jpg" style="width: 500px; height: 268px;" />
 * Return the matrix after applying k cyclic rotations to it.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/19/rod2.png" style="width: 421px; height: 191px;" />
 *
 * Input: grid = [[40,10],[30,20]], k = 1
 * Output: [[10,20],[40,30]]
 * Explanation: The figures above represent the grid at every state.
 *
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/10/ringofgrid5.png" style="width: 231px; height: 262px;" /> <img alt="" src="https://assets.leetcode.com/uploads/2021/06/10/ringofgrid6.png" style="width: 231px; height: 262px;" /> <img alt="" src="https://assets.leetcode.com/uploads/2021/06/10/ringofgrid7.png" style="width: 231px; height: 262px;" />
 *
 *
 * Input: grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]], k = 2
 * Output: [[3,4,8,12],[2,11,10,16],[1,7,6,15],[5,9,13,14]]
 * Explanation: The figures above represent the grid at every state.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	2 <= m, n <= 50
 * 	Both m and n are even integers.
 * 	1 <= grid[i][j] <=^ 5000
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cyclically-rotating-a-grid/
// discuss: https://leetcode.com/problems/cyclically-rotating-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut grid = grid;

        let m = grid.len();
        let n = grid[0].len();

        let num_rings = m.min(n) / 2;

        for i in 0..num_rings {
            let num_rotations = k % (2 * (m + n - 4 * i) - 4);
            for _ in 0..num_rotations {
                for j in i..n - i - 1 {
                    grid[i].swap(j, j + 1);
                }

                for j in i..m - i - 1 {
                    let tmp = grid[j][n - i - 1];
                    grid[j][n - i - 1] = grid[j + 1][n - i - 1];
                    grid[j + 1][n - i - 1] = tmp;
                }

                let mut j = n - i - 1;

                while j > i {
                    grid[m - i - 1].swap(j, j - 1);
                    j -= 1;
                }

                let mut j = m - i - 1;

                while j > i + 1 {
                    let tmp = grid[j][i];
                    grid[j][i] = grid[j - 1][i];
                    grid[j - 1][i] = tmp;
                    j -= 1;
                }
            }
        }

        grid
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1914_example_1() {
        let grid = vec![vec![40, 10], vec![30, 20]];
        let k = 1;

        let result = vec![vec![10, 20], vec![40, 30]];

        assert_eq!(Solution::rotate_grid(grid, k), result);
    }

    #[test]
    fn test_1914_example_2() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let k = 2;

        let result = vec![
            vec![3, 4, 8, 12],
            vec![2, 11, 10, 16],
            vec![1, 7, 6, 15],
            vec![5, 9, 13, 14],
        ];

        assert_eq!(Solution::rotate_grid(grid, k), result);
    }
}
