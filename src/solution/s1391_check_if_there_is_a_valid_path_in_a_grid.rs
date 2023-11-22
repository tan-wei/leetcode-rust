/**
 * [1391] Check if There is a Valid Path in a Grid
 *
 * You are given an m x n grid. Each cell of grid represents a street. The street of grid[i][j] can be:
 *
 * 	1 which means a street connecting the left cell and the right cell.
 * 	2 which means a street connecting the upper cell and the lower cell.
 * 	3 which means a street connecting the left cell and the lower cell.
 * 	4 which means a street connecting the right cell and the lower cell.
 * 	5 which means a street connecting the left cell and the upper cell.
 * 	6 which means a street connecting the right cell and the upper cell.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/main.png" style="width: 450px; height: 708px;" />
 * You will initially start at the street of the upper-left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1). The path should only follow the streets.
 * Notice that you are not allowed to change any street.
 * Return true if there is a valid path in the grid or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e1.png" style="width: 455px; height: 311px;" />
 * Input: grid = [[2,4,3],[6,5,2]]
 * Output: true
 * Explanation: As shown you can start at cell (0, 0) and visit all the cells of the grid to reach (m - 1, n - 1).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e2.png" style="width: 455px; height: 293px;" />
 * Input: grid = [[1,2,1],[1,2,1]]
 * Output: false
 * Explanation: As shown you the street at cell (0, 0) is not connected with any street of any other cell and you will get stuck at cell (0, 0)
 *
 * Example 3:
 *
 * Input: grid = [[1,1,2]]
 * Output: false
 * Explanation: You will get stuck at cell (0, 1) and you cannot reach cell (0, 2).
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 300
 * 	1 <= grid[i][j] <= 6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/
// discuss: https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let (m, n) = (grid.len(), grid[0].len());
        let mut flag = vec![vec![0; n]; m];
        let mut sk = vec![];

        sk.push((0, 0));
        flag[0][0] = 1;
        while let Some((u, v)) = sk.pop() {
            if u == m - 1 && v == n - 1 {
                return true;
            }
            for k in 0..4 {
                let (x, y) = (u as i32 + dirs[k][0], v as i32 + dirs[k][1]);
                if x < 0 || x == m as i32 || y < 0 || y == n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);

                if flag[x][y] == 1 {
                    continue;
                }
                if k == 0 {
                    if grid[u][v] != 1 && grid[u][v] != 4 && grid[u][v] != 6 {
                        continue;
                    }
                    if grid[x][y] != 1 && grid[x][y] != 3 && grid[x][y] != 5 {
                        continue;
                    }
                }
                if k == 1 {
                    if grid[u][v] != 2 && grid[u][v] != 3 && grid[u][v] != 4 {
                        continue;
                    }
                    if grid[x][y] != 2 && grid[x][y] != 5 && grid[x][y] != 6 {
                        continue;
                    }
                }
                if k == 2 {
                    if grid[u][v] != 1 && grid[u][v] != 3 && grid[u][v] != 5 {
                        continue;
                    }
                    if grid[x][y] != 1 && grid[x][y] != 4 && grid[x][y] != 6 {
                        continue;
                    }
                }
                if k == 3 {
                    if grid[u][v] != 2 && grid[u][v] != 5 && grid[u][v] != 6 {
                        continue;
                    }
                    if grid[x][y] != 2 && grid[x][y] != 3 && grid[x][y] != 4 {
                        continue;
                    }
                }

                flag[x][y] = 1;
                sk.push((x, y));
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1391_example_1() {
        let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];

        let result = true;

        assert_eq!(Solution::has_valid_path(grid), result);
    }

    #[test]
    fn test_1391_example_2() {
        let grid = vec![vec![1, 2, 1], vec![1, 2, 1]];

        let result = false;

        assert_eq!(Solution::has_valid_path(grid), result);
    }

    #[test]
    fn test_1391_example_3() {
        let grid = vec![vec![1, 1, 2]];

        let result = false;

        assert_eq!(Solution::has_valid_path(grid), result);
    }
}
