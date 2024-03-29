/**
 * [0832] Flipping an Image
 *
 * Given an n x n binary matrix image, flip the image horizontally, then invert it, and return the resulting image.
 * To flip an image horizontally means that each row of the image is reversed.
 *
 * 	For example, flipping [1,1,0] horizontally results in [0,1,1].
 *
 * To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0.
 *
 * 	For example, inverting [0,1,1] results in [1,0,0].
 *
 *  
 * Example 1:
 *
 * Input: image = [[1,1,0],[1,0,1],[0,0,0]]
 * Output: [[1,0,0],[0,1,0],[1,1,1]]
 * Explanation: First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
 * Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
 *
 * Example 2:
 *
 * Input: image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
 * Output: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
 * Explanation: First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
 * Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
 *
 *  
 * Constraints:
 *
 * 	n == image.length
 * 	n == image[i].length
 * 	1 <= n <= 20
 * 	images[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flipping-an-image/
// discuss: https://leetcode.com/problems/flipping-an-image/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .iter()
            .map(|row| row.iter().rev().map(|d| !d & 1).collect())
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0832_example_1() {
        let image = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let result = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];

        assert_eq!(Solution::flip_and_invert_image(image), result);
    }

    #[test]
    fn test_0832_example_2() {
        let image = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
        ];
        let result = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0],
        ];

        assert_eq!(Solution::flip_and_invert_image(image), result);
    }
}
