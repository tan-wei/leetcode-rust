/**
 * [0827] Making A Large Island
 *
 * You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.
 *
 * Return the size of the largest island in grid after applying this operation.
 *
 * An island is a 4-directionally connected group of 1s.
 *
 *  
 * Example 1:
 *
 *
 * Input: grid = [[1,0],[0,1]]
 * Output: 3
 * Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[1,1],[1,0]]
 * Output: 4
 * Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
 *
 * Example 3:
 *
 *
 * Input: grid = [[1,1],[1,1]]
 * Output: 4
 * Explanation: Can't change any 0 to 1, only one island with area = 4.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 500
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/making-a-large-island/
// discuss: https://leetcode.com/problems/making-a-large-island/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/making-a-large-island/discuss/1377761/Rust-solution
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ids = vec![vec![None; n]; n];
        let mut sizes = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 && ids[i][j].is_none() {
                    let size = Self::dfs_helper(&grid, &mut ids, (i, j), sizes.len());
                    sizes.push(size);
                }
            }
        }
        let mut result = *sizes.iter().max().unwrap_or(&0);
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let mut v = Vec::new();
                if *col == 0 {
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let i = i.wrapping_add(d[0]);
                        let j = j.wrapping_add(d[1]);
                        if i < n && j < n {
                            if let Some(idx) = ids[i][j] {
                                v.push(idx);
                            }
                        }
                    }
                }
                v.sort_unstable();
                v.dedup();
                result = result.max(1 + v.iter().map(|&k| sizes[k]).sum::<i32>());
            }
        }
        result
    }

    fn dfs_helper(
        grid: &[Vec<i32>],
        ids: &mut Vec<Vec<Option<usize>>>,
        p: (usize, usize),
        idx: usize,
    ) -> i32 {
        ids[p.0][p.1] = Some(idx);
        let mut result = 1;
        for d in [0, 1, 0, !0, 0].windows(2) {
            let i = p.0.wrapping_add(d[0]);
            let j = p.1.wrapping_add(d[1]);
            if i < grid.len() && j < grid.len() && grid[i][j] == 1 && ids[i][j].is_none() {
                result += Self::dfs_helper(grid, ids, (i, j), idx);
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
    fn test_0827_example_1() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let result = 3;

        assert_eq!(Solution::largest_island(grid), result);
    }

    #[test]
    fn test_0827_example_2() {
        let grid = vec![vec![1, 1], vec![1, 0]];
        let result = 4;

        assert_eq!(Solution::largest_island(grid), result);
    }

    #[test]
    fn test_0827_example_3() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        let result = 4;

        assert_eq!(Solution::largest_island(grid), result);
    }
}
