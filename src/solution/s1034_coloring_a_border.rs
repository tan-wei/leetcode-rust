/**
 * vec![1034] Coloring A Border
 *
 * You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.
 * Two squares belong to the same connected component if they have the same color and are next to each other in any of the 4 directions.
 * The border of a connected component is all the squares in the connected component that are either 4-directionally adjacent to a square not in the component, or on the boundary of the grid (the first or last row or column).
 * You should color the border of the connected component that contains the square grid[row][col] with color.
 * Return the final grid.
 *  
 * Example 1:
 * Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
 * Output: [[3,3],[3,2]]
 * Example 2:
 * Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
 * Output: [[1,3,3],[2,3,3]]
 * Example 3:
 * Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
 * Output: [[2,2,2],[2,1,2],[2,2,2]]
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 50
 * 	1 <= grid[i][j], color <= 1000
 * 	0 <= row < m
 * 	0 <= col < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coloring-a-border/
// discuss: https://leetcode.com/problems/coloring-a-border/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/coloring-a-border/solutions/894019/rust-translated-4ms-100/
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let cl = grid[row as usize][col as usize];
        let mut grid = grid;
        Self::dfs_helper(&mut grid, row, col, cl);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] < 0 {
                    grid[i][j] = color;
                }
            }
        }
        grid
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, color: i32) {
        if r < 0
            || r >= grid.len() as i32
            || c < 0
            || c >= grid[0].len() as i32
            || grid[r as usize][c as usize] != color
        {
            return;
        }
        grid[r as usize][c as usize] = -color;
        Self::dfs_helper(grid, r - 1, c, color);
        Self::dfs_helper(grid, r + 1, c, color);
        Self::dfs_helper(grid, r, c - 1, color);
        Self::dfs_helper(grid, r, c + 1, color);
        if r > 0
            && r < grid.len() as i32 - 1
            && c > 0
            && c < grid[0].len() as i32 - 1
            && color == (grid[r as usize - 1][c as usize]).abs()
            && color == (grid[r as usize + 1][c as usize]).abs()
            && color == (grid[r as usize][c as usize - 1]).abs()
            && color == (grid[r as usize][c as usize + 1]).abs()
        {
            grid[r as usize][c as usize] = color;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1034_example_1() {
        let grid = vec![vec![1, 1], vec![1, 2]];
        let row = 0;
        let col = 0;
        let color = 3;
        let result = vec![vec![3, 3], vec![3, 2]];

        assert_eq!(Solution::color_border(grid, row, col, color), result);
    }

    #[test]
    fn test_1034_example_2() {
        let grid = vec![vec![1, 2, 2], vec![2, 3, 2]];
        let row = 0;
        let col = 1;
        let color = 3;
        let result = vec![vec![1, 3, 3], vec![2, 3, 3]];

        assert_eq!(Solution::color_border(grid, row, col, color), result);
    }

    #[test]
    fn test_1034_example_3() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let row = 1;
        let col = 1;
        let color = 2;
        let result = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];

        assert_eq!(Solution::color_border(grid, row, col, color), result);
    }
}
