/**
 * [0892] Surface Area of 3D Shapes
 *
 * You are given an n x n grid where you have placed some 1 x 1 x 1 cubes. Each value v = grid[i][j] represents a tower of v cubes placed on top of cell (i, j).
 * After placing these cubes, you have decided to glue any directly adjacent cubes to each other, forming several irregular 3D shapes.
 * Return the total surface area of the resulting shapes.
 * Note: The bottom face of each shape counts toward its surface area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/08/tmp-grid2.jpg" style="width: 162px; height: 162px;" />
 * Input: grid = [[1,2],[3,4]]
 * Output: 34
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/08/tmp-grid4.jpg" style="width: 242px; height: 242px;" />
 * Input: grid = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: 32
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/08/tmp-grid5.jpg" style="width: 242px; height: 242px;" />
 * Input: grid = [[2,2,2],[2,1,2],[2,2,2]]
 * Output: 46
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

// problem: https://leetcode.com/problems/surface-area-of-3d-shapes/
// discuss: https://leetcode.com/problems/surface-area-of-3d-shapes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let l = grid.len() as i32;
        let mut result = 0;

        for (row, i) in grid.iter().zip(0..) {
            for (&val, j) in row.iter().zip(0..).filter(|(val, _)| **val > 0) {
                result += 2; // up adn down surfaces
                for (dx, dy) in dirs.iter().cloned() {
                    let (xx, yy) = (j + dx, i + dy);
                    result += match xx >= 0 && yy >= 0 && xx < l && yy < l {
                        true => (val - grid[yy as usize][xx as usize]).max(0),
                        false => val,
                    };
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
    fn test_0892_example_1() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let result = 34;

        assert_eq!(Solution::surface_area(grid), result);
    }

    #[test]
    fn test_0892_example_2() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let result = 32;

        assert_eq!(Solution::surface_area(grid), result);
    }

    #[test]
    fn test_0892_example_3() {
        let grid = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];
        let result = 46;

        assert_eq!(Solution::surface_area(grid), result);
    }
}
