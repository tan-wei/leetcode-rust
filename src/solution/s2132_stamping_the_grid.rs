/**
 * [2132] Stamping the Grid
 *
 * You are given an m x n binary matrix grid where each cell is either 0 (empty) or 1 (occupied).
 * You are then given stamps of size stampHeight x stampWidth. We want to fit the stamps such that they follow the given restrictions and requirements:
 * <ol>
 * 	Cover all the empty cells.
 * 	Do not cover any of the occupied cells.
 * 	We can put as many stamps as we want.
 * 	Stamps can overlap with each other.
 * 	Stamps are not allowed to be rotated.
 * 	Stamps must stay completely inside the grid.
 * </ol>
 * Return true if it is possible to fit the stamps while following the given restrictions and requirements. Otherwise, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/ex1.png" style="width: 180px; height: 237px;" />
 * Input: grid = [[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]], stampHeight = 4, stampWidth = 3
 * Output: true
 * Explanation: We have two overlapping stamps (labeled 1 and 2 in the image) that are able to cover all the empty cells.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/ex2.png" style="width: 170px; height: 179px;" />
 * Input: grid = [[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]], stampHeight = 2, stampWidth = 2
 * Output: false
 * Explanation: There is no way to fit the stamps onto all the empty cells without the stamps going outside the grid.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[r].length
 * 	1 <= m, n <= 10^5
 * 	1 <= m * n <= 2 * 10^5
 * 	grid[r][c] is either 0 or 1.
 * 	1 <= stampHeight, stampWidth <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stamping-the-grid/
// discuss: https://leetcode.com/problems/stamping-the-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2132_example_1() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let stamp_height = 4;
        let stamp_width = 3;

        let result = true;

        assert_eq!(
            Solution::possible_to_stamp(grid, stamp_height, stamp_width),
            result
        );
    }

    #[test]
    fn test_2132_example_2() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let stamp_height = 2;
        let stamp_width = 2;

        let result = false;

        assert_eq!(
            Solution::possible_to_stamp(grid, stamp_height, stamp_width),
            result
        );
    }
}
