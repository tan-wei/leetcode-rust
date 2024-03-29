/**
 * [1463] Cherry Pickup II
 *
 * You are given a rows x cols matrix grid representing a field of cherries where grid[i][j] represents the number of cherries that you can collect from the (i, j) cell.
 * You have two robots that can collect cherries for you:
 *
 * 	Robot #1 is located at the top-left corner (0, 0), and
 * 	Robot #2 is located at the top-right corner (0, cols - 1).
 *
 * Return the maximum number of cherries collection using both robots by following the rules below:
 *
 * 	From a cell (i, j), robots can move to cell (i + 1, j - 1), (i + 1, j), or (i + 1, j + 1).
 * 	When any robot passes through a cell, It picks up all cherries, and the cell becomes an empty cell.
 * 	When both robots stay in the same cell, only one takes the cherries.
 * 	Both robots cannot move outside of the grid at any moment.
 * 	Both robots should reach the bottom row in grid.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_1_1802.png" style="width: 374px; height: 501px;" />
 * Input: grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
 * Output: 24
 * Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
 * Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
 * Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
 * Total of cherries: 12 + 12 = 24.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/sample_2_1802.png" style="width: 500px; height: 452px;" />
 * Input: grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]
 * Output: 28
 * Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
 * Cherries taken by Robot #1, (1 + 9 + 5 + 2) = 17.
 * Cherries taken by Robot #2, (1 + 3 + 4 + 3) = 11.
 * Total of cherries: 17 + 11 = 28.
 *
 *  
 * Constraints:
 *
 * 	rows == grid.length
 * 	cols == grid[i].length
 * 	2 <= rows, cols <= 70
 * 	0 <= grid[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cherry-pickup-ii/
// discuss: https://leetcode.com/problems/cherry-pickup-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/cherry-pickup-ii/solutions/1676409/rust-dfs-memo/
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut memo: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; len_c]; len_c]; len_r];
        Self::dfs_helper(0, 0, len_c - 1, &grid, &mut memo)
    }

    fn dfs_helper(
        r: usize,
        c1: usize,
        c2: usize,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        if r == len_r {
            return 0;
        }
        if let Some(m) = memo[r][c1][c2] {
            return m;
        }
        let mut cherries: i32 = 0;
        for delta1 in -1..=1 as isize {
            for delta2 in -1..=1 as isize {
                let c1_nxt = c1 as isize + delta1;
                let c2_nxt = c2 as isize + delta2;
                if c1_nxt < 0 || c2_nxt < 0 || c1_nxt as usize >= len_c || c2_nxt as usize >= len_c
                {
                    continue;
                }
                cherries = std::cmp::max(
                    cherries,
                    Self::dfs_helper(r + 1, c1_nxt as usize, c2_nxt as usize, grid, memo),
                );
            }
        }
        cherries += if c1 == c2 {
            grid[r][c1]
        } else {
            grid[r][c1] + grid[r][c2]
        };
        memo[r][c1][c2] = Some(cherries);

        cherries
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1463_example_1() {
        let grid = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]];

        let result = 24;

        assert_eq!(Solution::cherry_pickup(grid), result);
    }

    #[test]
    fn test_1463_example_2() {
        let grid = vec![
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![2, 0, 0, 0, 0, 3, 0],
            vec![2, 0, 9, 0, 0, 0, 0],
            vec![0, 3, 0, 5, 4, 0, 0],
            vec![1, 0, 2, 3, 0, 0, 6],
        ];

        let result = 28;

        assert_eq!(Solution::cherry_pickup(grid), result);
    }
}
