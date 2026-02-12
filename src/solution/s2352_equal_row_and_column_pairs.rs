/**
 * [2352] Equal Row and Column Pairs
 *
 * Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.
 * A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/01/ex1.jpg" style="width: 150px; height: 153px;" />
 * Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
 * Output: 1
 * Explanation: There is 1 equal row and column pair:
 * - (Row 2, Column 1): [2,7,7]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/01/ex2.jpg" style="width: 200px; height: 209px;" />
 * Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
 * Output: 3
 * Explanation: There are 3 equal row and column pairs:
 * - (Row 0, Column 0): [3,1,2,2]
 * - (Row 2, Column 2): [2,4,2,2]
 * - (Row 3, Column 2): [2,4,2,2]
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 200
 * 	1 <= grid[i][j] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/equal-row-and-column-pairs/
// discuss: https://leetcode.com/problems/equal-row-and-column-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter().fold(0, |res, row| {
            res + grid.iter().enumerate().fold(0, |res_for_row, (ind, _)| {
                let column = grid.iter().map(|row| row[ind]).collect::<Vec<i32>>();
                res_for_row + (row == &column) as i32
            })
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2352_example_1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];

        let result = 1;

        assert_eq!(Solution::equal_pairs(grid), result);
    }

    #[test]
    fn test_2352_example_2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];

        let result = 3;

        assert_eq!(Solution::equal_pairs(grid), result);
    }
}
