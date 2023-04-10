/**
 * [1030] Matrix Cells in Distance Order
 *
 * You are given four integers row, cols, rCenter, and cCenter. There is a rows x cols matrix and you are on the cell with the coordinates (rCenter, cCenter).
 * Return the coordinates of all cells in the matrix, sorted by their distance from (rCenter, cCenter) from the smallest distance to the largest distance. You may return the answer in any order that satisfies this condition.
 * The distance between two cells (r1, c1) and (r2, c2) is |r1 - r2| + |c1 - c2|.
 *  
 * Example 1:
 *
 * Input: rows = 1, cols = 2, rCenter = 0, cCenter = 0
 * Output: [[0,0],[0,1]]
 * Explanation: The distances from (0, 0) to other cells are: [0,1]
 *
 * Example 2:
 *
 * Input: rows = 2, cols = 2, rCenter = 0, cCenter = 1
 * Output: [[0,1],[0,0],[1,1],[1,0]]
 * Explanation: The distances from (0, 1) to other cells are: [0,1,1,2]
 * The answer [[0,1],[1,1],[0,0],[1,0]] would also be accepted as correct.
 *
 * Example 3:
 *
 * Input: rows = 2, cols = 3, rCenter = 1, cCenter = 2
 * Output: [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
 * Explanation: The distances from (1, 2) to other cells are: [0,1,1,2,2,3]
 * There are other answers that would also be accepted as correct, such as [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]].
 *
 *  
 * Constraints:
 *
 * 	1 <= rows, cols <= 100
 * 	0 <= rCenter < rows
 * 	0 <= cCenter < cols
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/matrix-cells-in-distance-order/
// discuss: https://leetcode.com/problems/matrix-cells-in-distance-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..rows {
            for j in 0..cols {
                result.push(vec![i, j]);
            }
        }

        result.sort_unstable_by(|a, b| {
            ((a[0] - r_center).abs() + (a[1] - c_center).abs())
                .cmp(&((b[0] - r_center).abs() + (b[1] - c_center).abs()))
        });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1030_example_1() {
        let rows = 1;
        let cols = 2;
        let r_center = 0;
        let c_center = 0;
        let result = vec![vec![0, 0], vec![0, 1]];

        assert_eq!(
            Solution::all_cells_dist_order(rows, cols, r_center, c_center),
            result
        );
    }

    #[test]
    fn test_1030_example_2() {
        let rows = 2;
        let cols = 2;
        let r_center = 0;
        let c_center = 1;
        let result = vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]];

        assert_eq!(
            Solution::all_cells_dist_order(rows, cols, r_center, c_center),
            result
        );
    }

    #[test]
    fn test_1030_example_3() {
        let rows = 2;
        let cols = 3;
        let r_center = 1;
        let c_center = 2;
        let result = vec![
            vec![1, 2],
            vec![0, 2],
            vec![1, 1],
            vec![0, 1],
            vec![1, 0],
            vec![0, 0],
        ];

        assert_eq!(
            Solution::all_cells_dist_order(rows, cols, r_center, c_center),
            result
        );
    }
}
