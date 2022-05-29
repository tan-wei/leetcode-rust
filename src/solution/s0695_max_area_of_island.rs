/**
 * [0695] Max Area of Island
 *
 * You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.
 * The area of an island is the number of cells with a value 1 in the island.
 * Return the maximum area of an island in grid. If there is no island, return 0.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/01/maxarea1-grid.jpg" style="width: 500px; height: 310px;" />
 * Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
 * Output: 6
 * Explanation: The answer is not 11, because the island must be connected 4-directionally.
 *
 * Example 2:
 *
 * Input: grid = [[0,0,0,0,0,0,0,0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 50
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-area-of-island/
// discuss: https://leetcode.com/problems/max-area-of-island/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max_area = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != 0 {
                    max_area = max_area.max(Self::dfs_helper(&mut grid, x, y));
                }
            }
        }

        max_area
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        if grid[y][x] == 0 {
            0
        } else {
            grid[y][x] = 0;
            1 + if x > 0 {
                Self::dfs_helper(grid, x - 1, y)
            } else {
                0
            } + if x < grid[0].len() - 1 {
                Self::dfs_helper(grid, x + 1, y)
            } else {
                0
            } + if y > 0 {
                Self::dfs_helper(grid, x, y - 1)
            } else {
                0
            } + if y < grid.len() - 1 {
                Self::dfs_helper(grid, x, y + 1)
            } else {
                0
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0695_example_1() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        let result = 0;

        assert_eq!(Solution::max_area_of_island(grid), result);
    }
}
