/**
 * [0417] Pacific Atlantic Water Flow
 *
 * There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
 * The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
 * The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
 * Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg" style="width: 573px; height: 573px;" />
 * Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
 * Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
 *
 * Example 2:
 *
 * Input: heights = [[2,1],[1,2]]
 * Output: [[0,0],[0,1],[1,0],[1,1]]
 *
 *  
 * Constraints:
 *
 * 	m == heights.length
 * 	n == heights[r].length
 * 	1 <= m, n <= 200
 * 	0 <= heights[r][c] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pacific-atlantic-water-flow/
// discuss: https://leetcode.com/problems/pacific-atlantic-water-flow/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/pacific-atlantic-water-flow/discuss/1127040/Rust-DFS-solution
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() {
            return Vec::new();
        }
        let (m, n) = (heights.len(), heights[0].len());
        let mut p_reacheable = vec![vec![false; n]; m];
        let mut a_reacheable = vec![vec![false; n]; m];
        for i in 0..m {
            Self::dfs_helper(&mut p_reacheable, &heights, (i, 0));
            Self::dfs_helper(&mut a_reacheable, &heights, (i, n - 1));
        }
        for j in 0..n {
            Self::dfs_helper(&mut p_reacheable, &heights, (0, j));
            Self::dfs_helper(&mut a_reacheable, &heights, (m - 1, j));
        }
        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if p_reacheable[i][j] && a_reacheable[i][j] {
                    result.push([i as i32, j as i32].to_vec());
                }
            }
        }
        result
    }

    fn dfs_helper(reachable: &mut Vec<Vec<bool>>, heights: &[Vec<i32>], (i, j): (usize, usize)) {
        reachable[i][j] = true;
        for &(di, dj) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if (0..heights.len() as i32).contains(&ni)
                && (0..heights[0].len() as i32).contains(&nj)
                && !reachable[ni as usize][nj as usize]
                && heights[ni as usize][nj as usize] >= heights[i][j]
            {
                Self::dfs_helper(reachable, heights, (ni as usize, nj as usize));
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0417_example_1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let result = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];

        assert_eq!(Solution::pacific_atlantic(heights), result);
    }

    #[test]
    fn test_0417_example_2() {
        let heights = vec![vec![2, 1], vec![1, 2]];
        let result = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];

        assert_eq!(Solution::pacific_atlantic(heights), result);
    }
}
