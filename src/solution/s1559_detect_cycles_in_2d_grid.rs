/**
 * [1559] Detect Cycles in 2D Grid
 *
 * Given a 2D array of characters grid of size m x n, you need to find if there exists any cycle consisting of the same value in grid.
 * A cycle is a path of length 4 or more in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the same value of the current cell.
 * Also, you cannot move to the cell that you visited in your last move. For example, the cycle (1, 1) -> (1, 2) -> (1, 1) is invalid because from (1, 2) we visited (1, 1) which was the last visited cell.
 * Return true if any cycle of the same value exists in grid, otherwise, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/1.png" style="width: 231px; height: 152px;" />
 *
 * Input: grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
 * Output: true
 * Explanation: There are two valid cycles shown in different colors in the image below:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/11.png" style="width: 225px; height: 163px;" />
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/22.png" style="width: 236px; height: 154px;" />
 *
 * Input: grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
 * Output: true
 * Explanation: There is only one valid cycle highlighted in the image below:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/2.png" style="width: 229px; height: 157px;" />
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/3.png" style="width: 183px; height: 120px;" />
 *
 * Input: grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 500
 * 	grid consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/detect-cycles-in-2d-grid/
// discuss: https://leetcode.com/problems/detect-cycles-in-2d-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();

        let direction = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut seen = vec![vec![false; m]; n];

        for i in 0..n {
            for j in 0..m {
                if seen[i][j] {
                    continue;
                }

                let cc = grid[i][j];
                let mut stack = vec![(i as isize, j as isize, 10000, 10000)];

                while let Some((ci, cj, li, lj)) = stack.pop() {
                    seen[ci as usize][cj as usize] = true;
                    for &(ai, aj) in &direction {
                        let ni = ci + ai;
                        let nj = cj + aj;
                        if ni == li && nj == lj {
                            continue;
                        }
                        if ni < 0 || nj < 0 || n as isize <= ni || m as isize <= nj {
                            continue;
                        }
                        let nui = ni as usize;
                        let nuj = nj as usize;

                        if grid[nui][nuj] == cc {
                            if seen[nui][nuj] {
                                return true;
                            }
                            stack.push((ni, nj, ci, cj));
                        }
                    }
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1559_example_1() {
        let grid = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ];

        let result = true;

        assert_eq!(Solution::contains_cycle(grid), result);
    }

    #[test]
    fn test_1559_example_2() {
        let grid = vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ];

        let result = true;

        assert_eq!(Solution::contains_cycle(grid), result);
    }

    #[test]
    fn test_1559_example_3() {
        let grid = vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ];

        let result = false;

        assert_eq!(Solution::contains_cycle(grid), result);
    }
}
