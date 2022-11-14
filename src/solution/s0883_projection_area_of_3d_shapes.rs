/**
 * [0883] Projection Area of 3D Shapes
 *
 * You are given an n x n grid where we place some 1 x 1 x 1 cubes that are axis-aligned with the x, y, and z axes.
 * Each value v = grid[i][j] represents a tower of v cubes placed on top of the cell (i, j).
 * We view the projection of these cubes onto the xy, yz, and zx planes.
 * A projection is like a shadow, that maps our 3-dimensional figure to a 2-dimensional plane. We are viewing the "shadow" when looking at the cubes from the top, the front, and the side.
 * Return the total area of all three projections.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/02/shadow.png" style="width: 800px; height: 214px;" />
 * Input: grid = [[1,2],[3,4]]
 * Output: 17
 * Explanation: Here are the three projections ("shadows") of the shape made with each axis-aligned plane.
 *
 * Example 2:
 *
 * Input: grid = [[2]]
 * Output: 5
 *
 * Example 3:
 *
 * Input: grid = [[1,0],[0,2]]
 * Output: 8
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 50
 * 	0 <= grid[i][j] <= 50
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/projection-area-of-3d-shapes/
// discuss: https://leetcode.com/problems/projection-area-of-3d-shapes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let xy: i32 = grid
            .iter()
            .map(|p| p.iter().filter(|&&z| z > 0).count() as i32)
            .sum();
        let xz: i32 = grid.iter().map(|p| *p.iter().max().unwrap()).sum();
        let yz: i32 = (0..grid.len())
            .map(|i| grid.iter().map(|p| p[i]).max().unwrap())
            .sum();
        xy + xz + yz
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0883_example_1() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let result = 17;

        assert_eq!(Solution::projection_area(grid), result);
    }

    #[test]
    fn test_0883_example_2() {
        let grid = vec![vec![2]];
        let result = 5;

        assert_eq!(Solution::projection_area(grid), result);
    }

    #[test]
    fn test_0883_example_3() {
        let grid = vec![vec![1, 0], vec![0, 2]];
        let result = 8;

        assert_eq!(Solution::projection_area(grid), result);
    }
}
