/**
 * [1263] Minimum Moves to Move a Box to Their Target Location
 *
 * A storekeeper is a game in which the player pushes boxes around in a warehouse trying to get them to target locations.
 * The game is represented by an m x n grid of characters grid where each element is a wall, floor, or box.
 * Your task is to move the box 'B' to the target position 'T' under the following rules:
 *
 * 	The character 'S' represents the player. The player can move up, down, left, right in grid if it is a floor (empty cell).
 * 	The character '.' represents the floor which means a free cell to walk.
 * 	The character '#' represents the wall which means an obstacle (impossible to walk there).
 * 	There is only one box 'B' and one target cell 'T' in the grid.
 * 	The box can be moved to an adjacent free cell by standing next to the box and then moving in the direction of the box. This is a push.
 * 	The player cannot walk through the box.
 *
 * Return the minimum number of pushes to move the box to the target. If there is no way to reach the target, return -1.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/sample_1_1620.png" style="width: 500px; height: 335px;" />
 * Input: grid = vec![vec!["#","#","#","#","#","#"],
 *                ["#","T","#","#","#","#"],
 *                ["#",".",".","B",".","#"],
 *                ["#",".","#","#",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: 3
 * Explanation: We return only the number of times the box is pushed.
 * Example 2:
 *
 * Input: grid = [["#","#","#","#","#","#"],
 *                ["#","T","#","#","#","#"],
 *                ["#",".",".","B",".","#"],
 *                ["#","#","#","#",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: -1
 *
 * Example 3:
 *
 * Input: grid = [["#","#","#","#","#","#"],
 *                ["#","T",".",".","#","#"],
 *                ["#",".","#","B",".","#"],
 *                ["#",".",".",".",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: 5
 * Explanation: push the box down, left, left, up and up.
 *
 *
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 20
 * 	grid contains only characters '.', '#', 'S', 'T', or 'B'.
 * 	There is only one character 'S', 'B', and 'T' in the grid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/
// discuss: https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/solutions/2699481/rust-dijkstra/

const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const OBSTACLE: char = '#';
const BOX: char = 'B';
const PERSON: char = 'S';
const TARGET: char = 'T';

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = std::collections::HashSet::new();
        let (mut pr, mut pc) = (0, 0);
        let (mut br, mut bc) = (0, 0);
        let (mut tr, mut tc) = (0, 0);

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    BOX => {
                        br = r;
                        bc = c;
                    }
                    PERSON => {
                        pr = r;
                        pc = c;
                    }
                    TARGET => {
                        tr = r;
                        tc = c;
                    }
                    _ => {}
                }
            }
        }

        let mut pq = std::collections::BinaryHeap::new();
        pq.push((std::cmp::Reverse(0), br, bc, pr, pc));
        visited.insert((br, bc, pr, pc));

        while let Some((std::cmp::Reverse(cost), br, bc, pr, pc)) = pq.pop() {
            if (tr, tc) == (br, bc) {
                return cost;
            }

            for (dr, dc) in DIR.iter().copied() {
                let rx = pr as isize + dr;
                let cx = pc as isize + dc;

                // out of bounds: up/left
                if rx < 0 || cx < 0 {
                    continue;
                }

                let pr_new = rx as usize;
                let pc_new = cx as usize;

                // out of bounds: down/right
                if pr_new >= grid.len() || pc_new >= grid[pr_new].len() {
                    continue;
                }

                // we cannot move into a wall
                if grid[pr_new][pc_new] == OBSTACLE {
                    continue;
                }

                let mut cost_new = cost;
                let mut br_new = br;
                let mut bc_new = bc;

                // check if we are moving the box
                if (pr_new, pc_new) == (br_new, bc_new) {
                    let rx = br_new as isize + dr;
                    let cx = bc_new as isize + dc;

                    // cannot move the box outside the board
                    if rx < 0 || cx < 0 {
                        continue;
                    }

                    br_new = rx as usize;
                    bc_new = cx as usize;

                    // cannot move the box outside the board
                    if br_new >= grid.len() || bc_new >= grid[br_new].len() {
                        continue;
                    }

                    // cannot move the box into an obstacle
                    if grid[br_new][bc_new] == OBSTACLE {
                        continue;
                    }

                    // we've moved the box
                    cost_new += 1;
                }

                // Skip already visited combinations
                if !visited.insert((br_new, bc_new, pr_new, pc_new)) {
                    continue;
                }

                pq.push((std::cmp::Reverse(cost_new), br_new, bc_new, pr_new, pc_new));
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1263_example_1() {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];
        let result = 3;

        assert_eq!(Solution::min_push_box(grid), result);
    }

    #[test]
    fn test_1263_example_2() {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];

        let result = -1;

        assert_eq!(Solution::min_push_box(grid), result);
    }

    #[test]
    fn test_1263_example_3() {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '.', '.', '#', '#'],
            vec!['#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];

        let result = 5;

        assert_eq!(Solution::min_push_box(grid), result);
    }
}
