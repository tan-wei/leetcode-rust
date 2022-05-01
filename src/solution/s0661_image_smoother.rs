/**
 * [0661] Image Smoother
 *
 * An image smoother is a filter of the size 3 x 3 that can be applied to each cell of an image by rounding down the average of the cell and the eight surrounding cells (i.e., the average of the nine cells in the blue smoother). If one or more of the surrounding cells of a cell is not present, we do not consider it in the average (i.e., the average of the four cells in the red smoother).
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/smoother-grid.jpg" style="width: 493px; height: 493px;" />
 * Given an m x n integer matrix img representing the grayscale of an image, return the image after applying the smoother on each cell of it.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/smooth-grid.jpg" style="width: 613px; height: 253px;" />
 * Input: img = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[0,0,0],[0,0,0],[0,0,0]]
 * Explanation:
 * For the points (0,0), (0,2), (2,0), (2,2): floor(3/4) = floor(0.75) = 0
 * For the points (0,1), (1,0), (1,2), (2,1): floor(5/6) = floor(0.83333333) = 0
 * For the point (1,1): floor(8/9) = floor(0.88888889) = 0
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/smooth2-grid.jpg" style="width: 613px; height: 253px;" />
 * Input: img = [[100,200,100],[200,50,200],[100,200,100]]
 * Output: [[137,141,137],[141,138,141],[137,141,137]]
 * Explanation:
 * For the points (0,0), (0,2), (2,0), (2,2): floor((100+200+200+50)/4) = floor(137.5) = 137
 * For the points (0,1), (1,0), (1,2), (2,1): floor((200+200+50+200+100+100)/6) = floor(141.666667) = 141
 * For the point (1,1): floor((50+200+200+200+200+100+100+100+100)/9) = floor(138.888889) = 138
 *
 *  
 * Constraints:
 *
 * 	m == img.length
 * 	n == img[i].length
 * 	1 <= m, n <= 200
 * 	0 <= img[i][j] <= 255
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/image-smoother/
// discuss: https://leetcode.com/problems/image-smoother/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let directions = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (0, 0),
        ];
        let (n, m) = (img.len() as i32, img[0].len() as i32);
        let mut result = vec![vec![0; m as usize]; n as usize];
        for i in 0..n {
            for j in 0..m {
                let (mut num, mut sum) = (0, 0);
                for (new_i, new_j) in directions
                    .iter()
                    .map(|&(di, dj)| (i + di, j + dj))
                    .filter(|&(new_i, new_j)| new_i >= 0 && new_i < n && new_j >= 0 && new_j < m)
                {
                    num += 1;
                    sum += img[new_i as usize][new_j as usize];
                }
                result[i as usize][j as usize] = sum / num;
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
    fn test_0661_example_1() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let result = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];

        assert_eq!(Solution::image_smoother(img), result);
    }

    #[test]
    fn test_0661_example_2() {
        let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        let result = vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137],
        ];

        assert_eq!(Solution::image_smoother(img), result);
    }
}
