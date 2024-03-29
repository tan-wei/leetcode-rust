/**
 * [1091] Shortest Path in Binary Matrix
 *
 * Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
 * A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
 *
 * 	All the visited cells of the path are 0.
 * 	All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
 *
 * The length of a clear path is the number of visited cells of this path.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example1_1.png" style="width: 500px; height: 234px;" />
 * Input: grid = [[0,1],[1,0]]
 * Output: 2
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example2_1.png" style="height: 216px; width: 500px;" />
 * Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
 * Output: 4
 *
 * Example 3:
 *
 * Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 100
 * 	grid[i][j] is 0 or 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-in-binary-matrix/
// discuss: https://leetcode.com/problems/shortest-path-in-binary-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());
        let mut result = vec![vec![None; grid[0].len()]; grid.len()];
        let mut vd = std::collections::VecDeque::new();
        if grid[0][0] == 0 {
            vd.push_back(((0, 0), 1));
            result[0][0] = Some(1);
        }
        while let Some((p, len)) = vd.pop_front() {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if (0..r as i32).contains(&(p.0 + i)) && (0..c as i32).contains(&(p.1 + j)) {
                        let q = ((p.0 + i) as usize, (p.1 + j) as usize);
                        if grid[q.0][q.1] == 0 && result[q.0][q.1].is_none() {
                            result[q.0][q.1] = Some(len + 1);
                            vd.push_back(((p.0 + i, p.1 + j), len + 1));
                        }
                    }
                }
            }
        }
        result[r - 1][c - 1].unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1091_example_1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result = 2;

        assert_eq!(Solution::shortest_path_binary_matrix(grid), result);
    }

    #[test]
    fn test_1091_example_2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result = 4;

        assert_eq!(Solution::shortest_path_binary_matrix(grid), result);
    }

    #[test]
    fn test_1091_example_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result = -1;

        assert_eq!(Solution::shortest_path_binary_matrix(grid), result);
    }
}
