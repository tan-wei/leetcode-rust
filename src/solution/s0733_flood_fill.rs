/**
 * [0733] Flood Fill
 *
 * An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.
 * You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].
 * To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.
 * Return the modified image after performing the flood fill.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/flood1-grid.jpg" style="width: 613px; height: 253px;" />
 * Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
 * Output: [[2,2,2],[2,2,0],[2,0,1]]
 * Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
 * Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.
 *
 * Example 2:
 *
 * Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
 * Output: [[0,0,0],[0,0,0]]
 * Explanation: The starting pixel is already colored 0, so no changes are made to the image.
 *
 *  
 * Constraints:
 *
 * 	m == image.length
 * 	n == image[i].length
 * 	1 <= m, n <= 50
 * 	0 <= image[i][j], color < 2^16
 * 	0 <= sr < m
 * 	0 <= sc < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flood-fill/
// discuss: https://leetcode.com/problems/flood-fill/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut result = image;
        let (rows, cols) = (result.len() as i32, result[0].len() as i32);
        let old_color = result[sr as usize][sc as usize];
        let mut stack = vec![(sr, sc)];
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut visited = std::collections::HashSet::new();

        while let Some((x, y)) = stack.pop() {
            if !visited.insert((x, y)) {
                continue;
            }

            result[x as usize][y as usize] = color;

            for &(dx, dy) in dirs.iter() {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0
                    && new_x < rows
                    && new_y >= 0
                    && new_y < cols
                    && result[new_x as usize][new_y as usize] == old_color
                {
                    stack.push((new_x, new_y));
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
    fn test_0733_example_1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let result = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];

        assert_eq!(Solution::flood_fill(image, sr, sc, color), result);
    }

    #[test]
    fn test_0733_example_2() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr = 0;
        let sc = 0;
        let color = 0;
        let result = vec![vec![0, 0, 0], vec![0, 0, 0]];

        assert_eq!(Solution::flood_fill(image, sr, sc, color), result);
    }
}
