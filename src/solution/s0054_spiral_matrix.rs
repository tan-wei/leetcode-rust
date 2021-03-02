/**
 * [54] Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 10
 * 	-100 <= matrix[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix/
// discuss: https://leetcode.com/problems/spiral-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();

        if m == 0 {
            return vec![];
        }

        let n = matrix[0].len();

        let mut up = 0;
        let mut down = m - 1;
        let mut left = 0;
        let mut right = n - 1;

        let mut result = vec![];
        loop {
            for i in left..=right {
                result.push(matrix[up][i]);
            }
            up += 1;
            if down < up {
                break;
            }
            for i in up..=down {
                result.push(matrix[i][right]);
            }
            let r = right.overflowing_sub(1); // right is usize type, (right -1) may overflow
            if r.1 || r.0 < left {
                break;
            }
            right = r.0;
            for i in (left..=right).rev() {
                result.push(matrix[down][i]);
            }
            let r = down.overflowing_sub(1);
            if r.1 || r.0 < up {
                break;
            }
            down = r.0;
            for i in (up..=down).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
            if right < left {
                break;
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
    fn test_0054_example_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        assert_eq!(Solution::spiral_order(matrix), result);
    }

    #[test]
    fn test_0054_example_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        assert_eq!(Solution::spiral_order(matrix), result);
    }
}
