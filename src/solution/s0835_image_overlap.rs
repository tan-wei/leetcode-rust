/**
 * [0835] Image Overlap
 *
 * You are given two images, img1 and img2, represented as binary, square matrices of size n x n. A binary matrix has only 0s and 1s as values.
 * We translate one image however we choose by sliding all the 1 bits left, right, up, and/or down any number of units. We then place it on top of the other image. We can then calculate the overlap by counting the number of positions that have a 1 in both images.
 * Note also that a translation does not include any kind of rotation. Any 1 bits that are translated outside of the matrix borders are erased.
 * Return the largest possible overlap.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/overlap1.jpg" style="width: 450px; height: 231px;" />
 * Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
 * Output: 3
 * Explanation: We translate img1 to right by 1 unit and down by 1 unit.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step1.jpg" style="width: 450px; height: 105px;" />
 * The number of positions that have a 1 in both images is 3 (shown in red).
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step2.jpg" style="width: 450px; height: 231px;" />
 *
 * Example 2:
 *
 * Input: img1 = [[1]], img2 = [[1]]
 * Output: 1
 *
 * Example 3:
 *
 * Input: img1 = [[0]], img2 = [[0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == img1.length == img1[i].length
 * 	n == img2.length == img2[i].length
 * 	1 <= n <= 30
 * 	img1[i][j] is either 0 or 1.
 * 	img2[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/image-overlap/
// discuss: https://leetcode.com/problems/image-overlap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();

        let ones = (0..n)
            .flat_map(|y| (0..n).map(move |x| (y, x)))
            .filter(|(y, x)| img2[*y][*x] == 1)
            .collect::<Vec<_>>();

        (0..n)
            .flat_map(|y| (0..n).map(move |x| (y, x)))
            .filter(|(y, x)| img1[*y][*x] == 1)
            .flat_map(|(y1, x1)| ones.iter().copied().map(move |(y2, x2)| (y1, x1, y2, x2)))
            .fold(
                vec![vec![0; n * 2 - 1]; n * 2 - 1],
                |mut v, (y1, x1, y2, x2)| {
                    v[((n * 2 - 1) / 2) + y2 - y1][(n * 2 - 1) / 2 + x2 - x1] += 1;
                    v
                },
            )
            .into_iter()
            .flat_map(std::convert::identity)
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0835_example_1() {
        let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
        let result = 3;

        assert_eq!(Solution::largest_overlap(img1, img2), result);
    }

    #[test]
    fn test_0835_example_2() {
        let img1 = vec![vec![1]];
        let img2 = vec![vec![1]];
        let result = 1;

        assert_eq!(Solution::largest_overlap(img1, img2), result);
    }

    #[test]
    fn test_0835_example_3() {
        let img1 = vec![vec![0]];
        let img2 = vec![vec![0]];
        let result = 0;

        assert_eq!(Solution::largest_overlap(img1, img2), result);
    }
}
