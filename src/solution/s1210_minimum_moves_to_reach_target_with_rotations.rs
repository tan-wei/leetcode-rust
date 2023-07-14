/**
 * [1210] Minimum Moves to Reach Target with Rotations
 *
 * In an n*n grid, there is a snake that spans 2 cells and starts moving from the top left corner at (0, 0) and (0, 1). The grid has empty cells represented by zeros and blocked cells represented by ones. The snake wants to reach the lower right corner at (n-1, n-2) and (n-1, n-1).
 * In one move the snake can:
 *
 * 	Move one cell to the right if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
 * 	Move down one cell if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
 * 	Rotate clockwise if it's in a horizontal position and the two cells under it are both empty. In that case the snake moves from (r, c) and (r, c+1) to (r, c) and (r+1, c).<br />
 * 	<img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image-2.png" style="width: 300px; height: 134px;" />
 * 	Rotate counterclockwise if it's in a vertical position and the two cells to its right are both empty. In that case the snake moves from (r, c) and (r+1, c) to (r, c) and (r, c+1).<br />
 * 	<img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image-1.png" style="width: 300px; height: 121px;" />
 *
 * Return the minimum number of moves to reach the target.
 * If there is no way to reach the target, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image.png" style="width: 400px; height: 439px;" />
 *
 * Input: grid = [[0,0,0,0,0,1],
 *                [1,1,0,0,1,0],
 *                [0,0,0,0,1,1],
 *                [0,0,1,0,1,0],
 *                [0,1,1,0,0,0],
 *                [0,1,1,0,0,0]]
 * Output: 11
 * Explanation:
 * One possible solution is [right, right, rotate clockwise, right, down, down, down, down, rotate counterclockwise, right, down].
 *
 * Example 2:
 *
 * Input: grid = [[0,0,1,1,1,1],
 *                [0,0,0,0,1,1],
 *                [1,1,0,0,0,1],
 *                [1,1,1,0,0,1],
 *                [1,1,1,0,0,1],
 *                [1,1,1,0,0,0]]
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 100
 * 	0 <= grid[i][j] <= 1
 * 	It is guaranteed that the snake starts at empty cells.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/
// discuss: https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/solutions/875906/rust-translated-8ms-100/
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut steps = 0;
        let mut q = std::collections::VecDeque::<(i32, i32, bool)>::new();
        q.push_back((0, 0, false));
        let mut grid = grid;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (r, c, vertical) = q.pop_front().unwrap();
                if r == grid.len() as i32 - 1 && c == grid[0].len() as i32 - 2 {
                    return steps;
                }
                if grid[r as usize][c as usize] & (if vertical { 2 } else { 4 }) == 0 {
                    grid[r as usize][c as usize] |= if vertical { 2 } else { 4 };
                    if Self::can_go_down(&grid, r, c, vertical) {
                        q.push_back((r + 1, c, vertical));
                    }
                    if Self::can_go_right(&grid, r, c, vertical) {
                        q.push_back((r, c + 1, vertical));
                    }
                    if Self::can_rotate(&grid, r, c) {
                        q.push_back((r, c, !vertical));
                    }
                }
            }
            steps += 1;
        }
        -1
    }

    fn can_rotate(g: &[Vec<i32>], r: i32, c: i32) -> bool {
        r < g.len() as i32 - 1
            && c < g[0].len() as i32 - 1
            && (g[r as usize + 1][c as usize] & 1) == 0
            && (g[r as usize][c as usize + 1] & 1) == 0
            && (g[r as usize + 1][c as usize + 1] & 1) == 0
    }

    fn can_go_down(g: &[Vec<i32>], r: i32, c: i32, vertical: bool) -> bool {
        if vertical {
            r < g.len() as i32 - 2 && (g[r as usize + 2][c as usize] & 1) == 0
        } else {
            r < g.len() as i32 - 1
                && (g[r as usize + 1][c as usize] & 1) == 0
                && (g[r as usize + 1][c as usize + 1] & 1) == 0
        }
    }

    fn can_go_right(g: &[Vec<i32>], r: i32, c: i32, vertical: bool) -> bool {
        if !vertical {
            c < g[0].len() as i32 - 2 && (g[r as usize][c as usize + 2] & 1) == 0
        } else {
            c < g[0].len() as i32 - 1
                && (g[r as usize][c as usize + 1] & 1) == 0
                && (g[r as usize + 1][c as usize + 1] & 1) == 0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1210_example_1() {
        let grid = vec![
            vec![0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
        ];
        let result = 11;

        assert_eq!(Solution::minimum_moves(grid), result);
    }

    #[test]
    fn test_1210_example_2() {
        let grid = vec![
            vec![0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
            vec![1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 0],
        ];
        let result = 9;

        assert_eq!(Solution::minimum_moves(grid), result);
    }
}
