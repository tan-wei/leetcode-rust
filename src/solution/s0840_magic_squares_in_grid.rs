/**
 * [0840] Magic Squares In Grid
 *
 * A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9 such that each row, column, and both diagonals all have the same sum.
 * Given a row x col grid of integers, how many 3 x 3 "magic square" subgrids are there?  (Each subgrid is contiguous).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/11/magic_main.jpg" style="width: 322px; height: 242px;" />
 * Input: grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]]
 * Output: 1
 * Explanation:
 * The following subgrid is a 3 x 3 magic square:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/11/magic_valid.jpg" style="width: 242px; height: 242px;" />
 * while this one is not:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/11/magic_invalid.jpg" style="width: 242px; height: 242px;" />
 * In total, there is only one magic square inside the given grid.
 *
 * Example 2:
 *
 * Input: grid = [[8]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	row == grid.length
 * 	col == grid[i].length
 * 	1 <= row, col <= 10
 * 	0 <= grid[i][j] <= 15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/magic-squares-in-grid/
// discuss: https://leetcode.com/problems/magic-squares-in-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/magic-squares-in-grid/solutions/240726/rust-0ms-2-5mb/
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        if m < 3 || n < 3 {
            return 0;
        }
        let mut result = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if Self::is_magic_square(&grid, i, j) {
                    result += 1;
                }
            }
        }
        result
    }

    fn is_magic_square(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
        let mut vs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut row_sum = 0;
        for j in y..y + 3 {
            row_sum = 0;
            for i in x..x + 3 {
                let v = grid[j][i];
                if v > 9 || v < 1 {
                    return false;
                }
                vs[v as usize] = 0;

                row_sum += v;
            }
            if row_sum != 15 {
                return false;
            }
        }

        for v in vs {
            if v != 0 {
                return false;
            }
        }

        let mut col_sum = 0;
        for i in x..x + 3 {
            col_sum = 0;
            for j in y..y + 3 {
                col_sum += grid[j][i];
            }
            if col_sum != 15 {
                return false;
            }
        }

        if grid[y][x] + grid[y + 1][x + 1] + grid[y + 2][x + 2] != 15 {
            return false;
        }

        if grid[y][x + 2] + grid[y + 1][x + 1] + grid[y + 2][x] != 15 {
            return false;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0840_example_1() {
        let grid = vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]];
        let result = 1;

        assert_eq!(Solution::num_magic_squares_inside(grid), result);
    }

    #[test]
    fn test_0840_example_2() {
        let grid = vec![vec![8]];
        let result = 0;

        assert_eq!(Solution::num_magic_squares_inside(grid), result);
    }
}
