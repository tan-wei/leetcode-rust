/**
 * [1219] Path with Maximum Gold
 *
 * In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold in that cell, 0 if it is empty.
 * Return the maximum amount of gold you can collect under the conditions:
 *
 * 	Every time you are located in a cell you will collect all the gold in that cell.
 * 	From your position, you can walk one step to the left, right, up, or down.
 * 	You can't visit the same cell more than once.
 * 	Never visit a cell with 0 gold.
 * 	You can start and stop collecting gold from any position in the grid that has some gold.
 *
 *  
 * Example 1:
 *
 * Input: grid = [[0,6,0],[5,8,7],[0,9,0]]
 * Output: 24
 * Explanation:
 * [[0,6,0],
 *  [5,8,7],
 *  [0,9,0]]
 * Path to get the maximum gold, 9 -> 8 -> 7.
 *
 * Example 2:
 *
 * Input: grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
 * Output: 28
 * Explanation:
 * [[1,0,7],
 *  [2,0,6],
 *  [3,4,5],
 *  [0,3,0],
 *  [9,0,20]]
 * Path to get the maximum gold, 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 15
 * 	0 <= grid[i][j] <= 100
 * 	There are at most 25 cells containing gold.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/path-with-maximum-gold/
// discuss: https://leetcode.com/problems/path-with-maximum-gold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    result = std::cmp::max(result, Solution::dfs_helper(&mut grid, i, j));
                }
            }
        }
        result
    }

    pub fn dfs_helper(mut grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if grid[row][col] == 0 {
            return 0;
        }

        let original = grid[row][col];

        grid[row][col] = 0;

        let left = {
            if col == 0 {
                0
            } else {
                Solution::dfs_helper(&mut grid, row, col - 1)
            }
        };

        let right = {
            if col == grid[0].len() - 1 {
                0
            } else {
                Solution::dfs_helper(&mut grid, row, col + 1)
            }
        };

        let up = {
            if row == 0 {
                0
            } else {
                Solution::dfs_helper(&mut grid, row - 1, col)
            }
        };

        let down = {
            if row == grid.len() - 1 {
                0
            } else {
                Solution::dfs_helper(&mut grid, row + 1, col)
            }
        };

        grid[row][col] = original;

        let horizontal_max = std::cmp::max(left, right);
        let vertical_max = std::cmp::max(up, down);
        std::cmp::max(horizontal_max, vertical_max) + grid[row][col]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1219_example_1() {
        let grid = vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]];
        let result = 24;

        assert_eq!(Solution::get_maximum_gold(grid), result);
    }

    #[test]
    fn test_1219_example_2() {
        let grid = vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20],
        ];
        let result = 28;

        assert_eq!(Solution::get_maximum_gold(grid), result);
    }
}
