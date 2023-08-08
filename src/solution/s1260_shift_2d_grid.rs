/**
 * [1260] Shift 2D Grid
 *
 * Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
 * In one shift operation:
 *
 * 	Element at grid[i][j] moves to grid[i][j + 1].
 * 	Element at grid[i][n - 1] moves to grid[i + 1][0].
 * 	Element at grid[m - 1][n - 1] moves to grid[0][0].
 *
 * Return the 2D grid after applying shift operation k times.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/05/e1.png" style="width: 400px; height: 178px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
 * Output: [[9,1,2],[3,4,5],[6,7,8]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/05/e2.png" style="width: 400px; height: 166px;" />
 * Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
 * Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
 *
 * Example 3:
 *
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
 * Output: [[1,2,3],[4,5,6],[7,8,9]]
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m <= 50
 * 	1 <= n <= 50
 * 	-1000 <= grid[i][j] <= 1000
 * 	0 <= k <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shift-2d-grid/
// discuss: https://leetcode.com/problems/shift-2d-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let grid_len = rows * cols;

        grid.into_iter()
            .flatten()
            .cycle()
            .skip(grid_len - (k as usize) % grid_len)
            .take(grid_len)
            .collect::<Vec<_>>()
            .chunks(cols)
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1260_example_1() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let result = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

        assert_eq!(Solution::shift_grid(grid, k), result);
    }

    #[test]
    fn test_1260_example_2() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let k = 4;
        let result = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];

        assert_eq!(Solution::shift_grid(grid, k), result);
    }

    #[test]
    fn test_1260_example_3() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 9;
        let result = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(Solution::shift_grid(grid, k), result);
    }
}
