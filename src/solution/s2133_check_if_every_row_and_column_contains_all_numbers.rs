/**
 * [2133] Check if Every Row and Column Contains All Numbers
 *
 * An n x n matrix is valid if every row and every column contains all the integers from 1 to n (inclusive).
 * Given an n x n integer matrix matrix, return true if the matrix is valid. Otherwise, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/example1drawio.png" style="width: 250px; height: 251px;" />
 * Input: matrix = [[1,2,3],[3,1,2],[2,3,1]]
 * Output: true
 * Explanation: In this case, n = 3, and every row and column contains the numbers 1, 2, and 3.
 * Hence, we return true.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/example2drawio.png" style="width: 250px; height: 251px;" />
 * Input: matrix = [[1,1,1],[1,2,3],[1,2,3]]
 * Output: false
 * Explanation: In this case, n = 3, but the first row and the first column do not contain the numbers 2 or 3.
 * Hence, we return false.
 *
 *  
 * Constraints:
 *
 * 	n == matrix.length == matrix[i].length
 * 	1 <= n <= 100
 * 	1 <= matrix[i][j] <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/
// discuss: https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        (0..matrix.len()).all(|i| {
            (0..matrix.len())
                .fold(vec![(0_u8, 0_u8); matrix.len()], |mut mat, j| {
                    mat[(matrix[i][j] - 1) as usize].0 += 1;
                    mat[(matrix[j][i] - 1) as usize].1 += 1;
                    mat
                })
                .into_iter()
                .all(|lr| lr == (1, 1))
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2133_example_1() {
        let matrix = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];

        let result = true;

        assert_eq!(Solution::check_valid(matrix), result);
    }

    #[test]
    fn test_2133_example_2() {
        let matrix = vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]];

        let result = false;

        assert_eq!(Solution::check_valid(matrix), result);
    }
}
