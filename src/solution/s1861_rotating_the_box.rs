/**
 * [1861] Rotating the Box
 *
 * You are given an m x n matrix of characters boxGrid representing a side-view of a box. Each cell of the box is one of the following:
 *
 * 	A stone '#'
 * 	A stationary obstacle '*'
 * 	Empty '.'
 *
 * The box is rotated 90 degrees clockwise, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity does not affect the obstacles' positions, and the inertia from the box's rotation does not affect the stones' horizontal positions.
 * It is guaranteed that each stone in boxGrid rests on an obstacle, another stone, or the bottom of the box.
 * Return an n x m matrix representing the box after the rotation described above.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcodewithstones.png" style="width: 300px; height: 150px;" />
 *
 * Input: boxGrid = [["#",".","#"]]
 * Output: [["."],
 *          ["#"],
 *          ["#"]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode2withstones.png" style="width: 375px; height: 195px;" />
 *
 * Input: boxGrid = [["#",".","*","."],
 *               ["#","#","*","."]]
 * Output: [["#","."],
 *          ["#","#"],
 *          ["*","*"],
 *          [".","."]]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode3withstone.png" style="width: 400px; height: 218px;" />
 *
 * Input: boxGrid = [["#","#","*",".","*","."],
 *               ["#","#","#","*",".","."],
 *               ["#","#","#",".","#","."]]
 * Output: [[".","#","#"],
 *          [".","#","#"],
 *          ["#","#","*"],
 *          ["#","*","."],
 *          ["#",".","*"],
 *          ["#",".","."]]
 *
 *  
 * Constraints:
 *
 * 	m == boxGrid.length
 * 	n == boxGrid[i].length
 * 	1 <= m, n <= 500
 * 	boxGrid[i][j] is either '#', '*', or '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotating-the-box/
// discuss: https://leetcode.com/problems/rotating-the-box/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        let mut rotated = vec![vec!['.'; m]; n];

        // Rotate the box 90 degrees clockwise
        for (row_idx, row) in box_grid.iter().enumerate() {
            let mut empty_row = n;
            for (col_idx, &cell) in row.iter().enumerate().rev() {
                match &cell {
                    '#' => {
                        empty_row -= 1;
                        rotated[empty_row][m - row_idx - 1] = cell;
                    }
                    '*' => {
                        rotated[col_idx][m - row_idx - 1] = '*';
                        empty_row = col_idx;
                    }
                    _ => {}
                }
            }
        }

        rotated
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1861_example_1() {
        let box_grid = vec![vec!['#', '.', '#']];

        let result = vec![vec!['.'], vec!['#'], vec!['#']];

        assert_eq!(Solution::rotate_the_box(box_grid), result);
    }

    #[test]
    fn test_1861_example_2() {
        let box_grid = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];

        let result = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];

        assert_eq!(Solution::rotate_the_box(box_grid), result);
    }

    #[test]
    fn test_1861_example_3() {
        let box_grid = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];

        let result = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];

        assert_eq!(Solution::rotate_the_box(box_grid), result);
    }
}
