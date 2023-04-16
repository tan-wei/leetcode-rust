/**
 * [1036] Escape a Large Maze
 *
 * There is a 1 million by 1 million grid on an XY-plane, and the coordinates of each grid square are (x, y).
 * We start at the source = [sx, sy] square and want to reach the target = [tx, ty] square. There is also an array of blocked squares, where each blocked[i] = [xi, yi] represents a blocked square with coordinates (xi, yi).
 * Each move, we can walk one square north, east, south, or west if the square is not in the array of blocked squares. We are also not allowed to walk outside of the grid.
 * Return true if and only if it is possible to reach the target square from the source square through a sequence of valid moves.
 *  
 * Example 1:
 *
 * Input: blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
 * Output: false
 * Explanation: The target square is inaccessible starting from the source square because we cannot move.
 * We cannot move north or east because those squares are blocked.
 * We cannot move south or west because we cannot go outside of the grid.
 *
 * Example 2:
 *
 * Input: blocked = [], source = [0,0], target = [999999,999999]
 * Output: true
 * Explanation: Because there are no blocked cells, it is possible to reach the target square.
 *
 *  
 * Constraints:
 *
 * 	0 <= blocked.length <= 200
 * 	blocked[i].length == 2
 * 	0 <= xi, yi < 10^6
 * 	source.length == target.length == 2
 * 	0 <= sx, sy, tx, ty < 10^6
 * 	source != target
 * 	It is guaranteed that source and target are not blocked.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/escape-a-large-maze/
// discuss: https://leetcode.com/problems/escape-a-large-maze/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/escape-a-large-maze/solutions/874695/rust-translated-8ms-100/
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut visited_source = blocked
            .iter()
            .filter(|&v| (v[0] - source[0]).abs() + (v[1] - source[1]).abs() < 400)
            .map(|v| (v[0], v[1]))
            .collect::<std::collections::HashSet<(i32, i32)>>();
        let mut visited_target = blocked
            .iter()
            .filter(|&v| (v[0] - target[0]).abs() + (v[1] - target[1]).abs() < 400)
            .map(|v| (v[0], v[1]))
            .collect::<std::collections::HashSet<(i32, i32)>>();
        let source_blocks = visited_source.len();
        let target_blocks = visited_target.len();
        let source = (source[0], source[1]);
        let target = (target[0], target[1]);
        Self::bfs_helper(&mut visited_source, source, target, source_blocks)
            && Self::bfs_helper(&mut visited_target, target, source, target_blocks)
    }

    fn bfs_helper(
        visited: &mut std::collections::HashSet<(i32, i32)>,
        source: (i32, i32),
        target: (i32, i32),
        blocks: usize,
    ) -> bool {
        const DIRS: [i32; 5] = [0, 1, 0, -1, 0];
        let mut queue: Vec<(i32, i32)> = vec![source];
        while !queue.is_empty() && queue.len() <= blocks {
            let mut temp = Vec::<(i32, i32)>::new();
            for &(x0, y0) in &queue {
                for d in 0..4 {
                    let x = x0 + DIRS[d];
                    let y = y0 + DIRS[d + 1];
                    if !(x >= 0 && x < 1_000_000 && y >= 0 && y < 1_000_000) {
                        continue;
                    }
                    if x == target.0 && y == target.1 {
                        return true;
                    }
                    if visited.insert((x, y)) {
                        temp.push((x, y));
                    }
                }
            }
            std::mem::swap(&mut queue, &mut temp);
        }
        !queue.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1036_example_1() {
        let blocked = vec![vec![0, 1], vec![1, 0]];
        let source = vec![0, 0];
        let target = vec![0, 2];
        let result = false;

        assert_eq!(
            Solution::is_escape_possible(blocked, source, target),
            result
        );
    }

    #[test]
    fn test_1036_example_2() {
        let blocked = vec![];
        let source = vec![0, 0];
        let target = vec![999999, 999999];
        let result = true;

        assert_eq!(
            Solution::is_escape_possible(blocked, source, target),
            result
        );
    }
}
