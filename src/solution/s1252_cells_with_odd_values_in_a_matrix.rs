/**
 * [1252] Cells with Odd Values in a Matrix
 *
 * There is an m x n matrix that is initialized to all 0's. There is also a 2D array indices where each indices[i] = [ri, ci] represents a 0-indexed location to perform some increment operations on the matrix.
 * For each location indices[i], do both of the following:
 * <ol>
 * 	Increment all the cells on row ri.
 * 	Increment all the cells on column ci.
 * </ol>
 * Given m, n, and indices, return the number of odd-valued cells in the matrix after applying the increment to all locations in indices.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/30/e1.png" style="width: 600px; height: 118px;" />
 * Input: m = 2, n = 3, indices = [[0,1],[1,1]]
 * Output: 6
 * Explanation: Initial matrix = [[0,0,0],[0,0,0]].
 * After applying first increment it becomes [[1,2,1],[0,1,0]].
 * The final matrix is [[1,3,1],[1,3,1]], which contains 6 odd numbers.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/30/e2.png" style="width: 600px; height: 150px;" />
 * Input: m = 2, n = 2, indices = [[1,1],[0,0]]
 * Output: 0
 * Explanation: Final matrix = [[2,2],[2,2]]. There are no odd numbers in the final matrix.
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 50
 * 	1 <= indices.length <= 100
 * 	0 <= ri < m
 * 	0 <= ci < n
 *
 *  
 * Follow up: Could you solve this in O(n + m + indices.length) time with only O(n + m) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
// discuss: https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];

        indices.iter().for_each(|point| {
            rows[point[0] as usize] += 1;
            cols[point[1] as usize] += 1;
        });

        (0..m as usize).for_each(|row| {
            (0..n as usize).for_each(|col| {
                result += (rows[row] + cols[col]) % 2;
            })
        });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1252_example_1() {
        let m = 2;
        let n = 3;
        let indices = vec![vec![0, 1], vec![1, 1]];
        let result = 6;

        assert_eq!(Solution::odd_cells(m, n, indices), result);
    }

    #[test]
    fn test_1252_example_2() {
        let m = 2;
        let n = 2;
        let indices = vec![vec![1, 1], vec![0, 0]];
        let result = 0;

        assert_eq!(Solution::odd_cells(m, n, indices), result);
    }
}
