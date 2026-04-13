/**
 * [2428] Maximum Sum of an Hourglass
 *
 * You are given an m x n integer matrix grid.
 * We define an hourglass as a part of the matrix with the following form:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/img.jpg" style="width: 243px; height: 243px;" />
 * Return the maximum sum of the elements of an hourglass.
 * Note that an hourglass cannot be rotated and must be entirely contained within the matrix.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/1.jpg" style="width: 323px; height: 323px;" />
 * Input: grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
 * Output: 30
 * Explanation: The cells shown above represent the hourglass with the maximum sum: 6 + 2 + 1 + 2 + 9 + 2 + 8 = 30.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/2.jpg" style="width: 243px; height: 243px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: 35
 * Explanation: There is only one hourglass in the matrix, with the sum: 1 + 2 + 3 + 5 + 7 + 8 + 9 = 35.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	3 <= m, n <= 150
 * 	0 <= grid[i][j] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-an-hourglass/
// discuss: https://leetcode.com/problems/maximum-sum-of-an-hourglass/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        // Credit: https://leetcode.com/problems/maximum-sum-of-an-hourglass/solutions/2648800/rust-windows-with-comments-by-wallicent-9zsl/
        let ends: Vec<Vec<i32>> = grid
            .iter()
            .map(|row| row.windows(3).map(|e| e.iter().sum()).collect())
            .collect();
        let middles: Vec<Vec<i32>> = grid
            .iter()
            .map(|row| row.windows(3).map(|e| e[1]).collect())
            .collect();
        ends.iter()
            .zip(middles.iter().skip(1))
            .zip(ends.iter().skip(2))
            .flat_map(|((top, middle), bottom)| {
                top.iter()
                    .zip(middle.iter())
                    .zip(bottom.iter())
                    .map(|((t, m), b)| *t + *m + *b)
            })
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2428_example_1() {
        let grid = vec![
            vec![6, 2, 1, 3],
            vec![4, 2, 1, 5],
            vec![9, 2, 8, 7],
            vec![4, 1, 2, 9],
        ];

        let result = 30;

        assert_eq!(Solution::max_sum(grid), result);
    }

    #[test]
    fn test_2428_example_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = 35;

        assert_eq!(Solution::max_sum(grid), result);
    }
}
