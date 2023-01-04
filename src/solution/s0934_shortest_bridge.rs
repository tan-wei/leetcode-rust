/**
 * [0934] Shortest Bridge
 *
 * You are given an n x n binary matrix grid where 1 represents land and 0 represents water.
 * An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.
 * You may change 0's to 1's to connect the two islands to form one island.
 * Return the smallest number of 0's you must flip to connect the two islands.
 *  
 * Example 1:
 *
 * Input: grid = [[0,1],[1,0]]
 * Output: 1
 *
 * Example 2:
 *
 * Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
 * Output: 2
 *
 * Example 3:
 *
 * Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	2 <= n <= 100
 * 	grid[i][j] is either 0 or 1.
 * 	There are exactly two islands in grid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-bridge/
// discuss: https://leetcode.com/problems/shortest-bridge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut found = false;
        let mut queue = std::collections::VecDeque::<(i32, i32)>::new();
        let mut level = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && !found {
                    Self::dfs_helper(&mut grid, i as i32, j as i32, m, n);
                    found = true;
                }
                if found && grid[i][j] == 1 {
                    queue.push_back((i as i32, j as i32));
                }
            }
        }
        const DIRS: [i32; 5] = [0, 1, 0, -1, 0];
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let pos = queue.pop_front().unwrap();
                for i in 0..4 {
                    let row = pos.0 + DIRS[i];
                    let col = pos.1 + DIRS[i + 1];

                    if row >= 0 && row < m as i32 && col >= 0 && col < n as i32 {
                        match grid[row as usize][col as usize] {
                            2 => return level,
                            1 => continue,
                            0 => {
                                grid[row as usize][col as usize] = 1;
                                queue.push_back((row, col));
                            }
                            _ => {}
                        }
                    }
                }
            }
            level += 1;
        }
        -1
    }

    fn dfs_helper(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, m: usize, n: usize) {
        grid[row as usize][col as usize] = 2;
        if row > 0 && grid[row as usize - 1][col as usize] == 1 {
            Self::dfs_helper(grid, row - 1, col, m, n);
        }
        if row + 1 < m as i32 && grid[row as usize + 1][col as usize] == 1 {
            Self::dfs_helper(grid, row + 1, col, m, n);
        }
        if col > 0 && grid[row as usize][col as usize - 1] == 1 {
            Self::dfs_helper(grid, row, col - 1, m, n);
        }
        if col + 1 < n as i32 && grid[row as usize][col as usize + 1] == 1 {
            Self::dfs_helper(grid, row, col + 1, m, n);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0934_example_1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result = 1;

        assert_eq!(Solution::shortest_bridge(grid), result);
    }

    #[test]
    fn test_0934_example_2() {
        let grid = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let result = 2;

        assert_eq!(Solution::shortest_bridge(grid), result);
    }

    #[test]
    fn test_0934_example_3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let result = 1;

        assert_eq!(Solution::shortest_bridge(grid), result);
    }
}
