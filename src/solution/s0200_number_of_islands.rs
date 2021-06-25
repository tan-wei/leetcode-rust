/**
 * [200] Number of Islands
 *
 * Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
 * An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *  
 * Example 1:
 *
 * Input: grid = [
 *   ["1","1","1","1","0"],
 *   ["1","1","0","1","0"],
 *   ["1","1","0","0","0"],
 *   ["0","0","0","0","0"]
 * ]
 * Output: 1
 *
 * Example 2:
 *
 * Input: grid = [
 *   ["1","1","0","0","0"],
 *   ["1","1","0","0","0"],
 *   ["0","0","1","0","0"],
 *   ["0","0","0","1","1"]
 * ]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 300
 * 	grid[i][j] is '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-islands/
// discuss: https://leetcode.com/problems/number-of-islands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn helper(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if (0..grid.len()).contains(&i) && (0..grid[0].len()).contains(&j) {
                if grid[i][j] == '1' {
                    grid[i][j] = '0';
                    if i >= 1 {
                        helper(grid, i - 1, j);
                    }

                    if i + 1 < grid.len() {
                        helper(grid, i + 1, j);
                    }

                    if j >= 1 {
                        helper(grid, i, j - 1);
                    }

                    if j + 1 < grid[0].len() {
                        helper(grid, i, j + 1);
                    }
                }
            }
        }

        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let mut result = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    result += 1;
                    helper(&mut grid, i, j);
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0200_example_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let result = 1;

        assert_eq!(Solution::num_islands(grid), result);
    }

    #[test]
    fn test_0200_example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let result = 3;

        assert_eq!(Solution::num_islands(grid), result);
    }
}
