/**
 * [0994] Rotting Oranges
 *
 * You are given an m x n grid where each cell can have one of three values:
 *
 * 	0 representing an empty cell,
 * 	1 representing a fresh orange, or
 * 	2 representing a rotten orange.
 *
 * Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
 * Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/16/oranges.png" style="width: 650px; height: 137px;" />
 * Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
 * Output: 4
 *
 * Example 2:
 *
 * Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
 * Output: -1
 * Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
 *
 * Example 3:
 *
 * Input: grid = [[0,2]]
 * Output: 0
 * Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 10
 * 	grid[i][j] is 0, 1, or 2.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotting-oranges/
// discuss: https://leetcode.com/problems/rotting-oranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];

        let (m, n) = (grid.len(), grid[0].len());
        let mut grid = grid;

        let (mut count, mut result) = (0, 0);
        let mut q = vec![];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                }
                if grid[i][j] == 2 {
                    q.push((i, j));
                }
            }
        }

        while count > 0 && q.is_empty() == false {
            let mut temp = vec![];
            result += 1;
            for (i, j) in q {
                for d in dirs {
                    let (u, v) = (i as i32 + d[0], j as i32 + d[1]);

                    if u < 0 || u == m as i32 || v < 0 || v == n as i32 {
                        continue;
                    }

                    let (u, v) = (u as usize, v as usize);
                    if grid[u][v] != 1 {
                        continue;
                    }

                    count -= 1;
                    grid[u][v] = 2;
                    temp.push((u, v));
                }
            }
            q = temp;
        }

        if count > 0 {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0994_example_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let result = 4;

        assert_eq!(Solution::oranges_rotting(grid), result);
    }

    #[test]
    fn test_0994_example_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        let result = -1;

        assert_eq!(Solution::oranges_rotting(grid), result);
    }

    #[test]
    fn test_0994_example_3() {
        let grid = vec![vec![0, 2]];
        let result = 0;

        assert_eq!(Solution::oranges_rotting(grid), result);
    }
}
