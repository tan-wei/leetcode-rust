/**
 * [1572] Matrix Diagonal Sum
 *
 * Given a square matrix mat, return the sum of the matrix diagonals.
 * Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/14/sample_1911.png" style="width: 336px; height: 174px;" />
 * Input: mat = [[1,2,3],
 *               [4,5,6],
 *               [7,8,9]]
 * Output: 25
 * Explanation: Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
 * Notice that element mat[1][1] = 5 is counted only once.
 *
 * Example 2:
 *
 * Input: mat = [[1,1,1,1],
 *               [1,1,1,1],
 *               [1,1,1,1],
 *               [1,1,1,1]]
 * Output: 8
 *
 * Example 3:
 *
 * Input: mat = [[5]]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	n == mat.length == mat[i].length
 * 	1 <= n <= 100
 * 	1 <= mat[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/matrix-diagonal-sum/
// discuss: https://leetcode.com/problems/matrix-diagonal-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        mat.iter().enumerate().fold(0, |sum, (i, row)| {
            sum + row
                .iter()
                .enumerate()
                .filter_map(|(j, n)| (i + j == mat.len() - 1 || i == j).then(|| n))
                .sum::<i32>()
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1572_example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = 25;

        assert_eq!(Solution::diagonal_sum(mat), result);
    }

    #[test]
    fn test_1572_example_2() {
        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];

        let result = 8;

        assert_eq!(Solution::diagonal_sum(mat), result);
    }

    #[test]
    fn test_1572_example_3() {
        let mat = vec![vec![5]];

        let result = 5;

        assert_eq!(Solution::diagonal_sum(mat), result);
    }
}
