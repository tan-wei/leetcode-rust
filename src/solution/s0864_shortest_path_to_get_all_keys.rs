/**
 * [0864] Shortest Path to Get All Keys
 *
 * You are given an m x n grid grid where:
 *
 * 	'.' is an empty cell.
 * 	'#' is a wall.
 * 	'@' is the starting point.
 * 	Lowercase letters represent keys.
 * 	Uppercase letters represent locks.
 *
 * You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.
 * If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.
 * For some <font face="monospace">1 <= k <= 6</font>, there is exactly one lowercase and one uppercase letter of the first k letters of the English alphabet in the grid. This means that there is exactly one key for each lock, and one lock for each key; and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.
 * Return the lowest number of moves to acquire all keys. If it is impossible, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@.a..","###.#","b.A.B"]
 * Output: 8
 * Explanation: Note that the goal is to obtain all the keys not to open all the locks.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-key2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@..aA","..B#.","....b"]
 * Output: 6
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys3.jpg" style="width: 244px; height: 85px;" />
 * Input: grid = ["@Aa"]
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 30
 * 	grid[i][j] is either an English letter, '.', '#', or '@'.
 * 	The number of keys in the grid is in the range [1, 6].
 * 	Each key in the grid is unique.
 * 	Each key in the grid has a matching lock.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-to-get-all-keys/
// discuss: https://leetcode.com/problems/shortest-path-to-get-all-keys/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/shortest-path-to-get-all-keys/solutions/839020/rust-translated-24ms-100/

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    i: i32,
    j: i32,
    keys: i32,
}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        const DIRS: [i32; 5] = [0, -1, 0, 1, 0];

        let m = grid.len();
        let n = grid[0].len();
        let mut i = -1i32;
        let mut j = -1i32;
        let mut max = 0;
        for x in 0..m {
            for y in 0..n {
                let c = grid[x].as_bytes()[y];
                if c == b'@' {
                    i = x as i32;
                    j = y as i32;
                } else if c >= b'a' && c <= b'f' {
                    max = std::cmp::max(max, c - b'a' + 1);
                }
            }
        }

        let mut all_keys = (1 << max) - 1;

        let mut visited = std::collections::HashSet::<State>::new();
        let mut queue = std::collections::VecDeque::<State>::new();
        let start = State { i, j, keys: 0 };
        visited.insert(start.clone());
        queue.push_back(start);
        let mut step = 0;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                size -= 1;
                let curr = queue.pop_front().unwrap();
                if curr.keys == all_keys {
                    return step;
                }
                for k in 0..4 {
                    let x = curr.i + DIRS[k];
                    let y = curr.j + DIRS[k + 1];
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let c = grid[x as usize].as_bytes()[y as usize];
                    if c == b'#' {
                        continue;
                    }
                    if c >= b'A' && c <= b'F' && (curr.keys & (1 << (c - b'A'))) == 0 {
                        continue;
                    }
                    let mut keys = curr.keys;
                    if c >= b'a' && c <= b'f' {
                        keys |= (1 << (c - b'a'));
                    }
                    let next = State { i: x, j: y, keys };
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(next.clone());
                    queue.push_back(next);
                }
            }
            step += 1;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0864_example_1() {
        let grid = vec_string!["@.a..", "###.#", "b.A.B"];
        let result = 8;

        assert_eq!(Solution::shortest_path_all_keys(grid), result);
    }

    #[test]
    fn test_0864_example_2() {
        let grid = vec_string!["@..aA", "..B#.", "....b"];
        let result = 6;

        assert_eq!(Solution::shortest_path_all_keys(grid), result);
    }

    #[test]
    fn test_0864_example_3() {
        let grid = vec_string!["@Aa"];
        let result = -1;

        assert_eq!(Solution::shortest_path_all_keys(grid), result);
    }
}
