/**
 * [0931] Minimum Falling Path Sum
 *
 * Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.
 * A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/failing1-grid.jpg" style="width: 499px; height: 500px;" />
 * Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
 * Output: 13
 * Explanation: There are two falling paths with a minimum sum as shown.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/failing2-grid.jpg" style="width: 164px; height: 365px;" />
 * Input: matrix = [[-19,57],[-40,-5]]
 * Output: -59
 * Explanation: The falling path with a minimum sum is shown.
 *
 *  
 * Constraints:
 *
 * 	n == matrix.length == matrix[i].length
 * 	1 <= n <= 100
 * 	-100 <= matrix[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-falling-path-sum/
// discuss: https://leetcode.com/problems/minimum-falling-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let (w, h) = (matrix[0].len(), matrix.len());
        let mut v = vec![vec![0; w]; h];

        for ir in (0..matrix.len()).rev() {
            for ic in 0..matrix[0].len() {
                v[ir][ic] = (if ir + 1 >= h {
                    0
                } else {
                    (if ic == 0 { i32::MAX } else { v[ir + 1][ic - 1] })
                        .min(v[ir + 1][ic])
                        .min(if ic + 1 >= w {
                            i32::MAX
                        } else {
                            v[ir + 1][ic + 1]
                        })
                }) + matrix[ir][ic];
            }
        }
        *v[0].iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0931_example_1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        let result = 13;

        assert_eq!(Solution::min_falling_path_sum(matrix), result);
    }

    #[test]
    fn test_0931_example_2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        let result = -59;

        assert_eq!(Solution::min_falling_path_sum(matrix), result);
    }
}
