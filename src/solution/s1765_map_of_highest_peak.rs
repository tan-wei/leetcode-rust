/**
 * [1765] Map of Highest Peak
 *
 * You are given an integer matrix isWater of size m x n that represents a map of land and water cells.
 *
 * 	If isWater[i][j] == 0, cell (i, j) is a land cell.
 * 	If isWater[i][j] == 1, cell (i, j) is a water cell.
 *
 * You must assign each cell a height in a way that follows these rules:
 *
 * 	The height of each cell must be non-negative.
 * 	If the cell is a water cell, its height must be 0.
 * 	Any two adjacent cells must have an absolute height difference of at most 1. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).
 *
 * Find an assignment of heights such that the maximum height in the matrix is maximized.
 * Return an integer matrix height of size m x n where height[i][j] is cell (i, j)'s height. If there are multiple solutions, return any of them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82045-am.png" style="width: 220px; height: 219px;" />
 *
 * Input: isWater = [[0,1],[0,0]]
 * Output: [[1,0],[2,1]]
 * Explanation: The image shows the assigned heights of each cell.
 * The blue cell is the water cell, and the green cells are the land cells.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82050-am.png" style="width: 300px; height: 296px;" />
 *
 * Input: isWater = [[0,0,1],[1,0,0],[0,0,0]]
 * Output: [[1,1,0],[0,1,1],[1,2,2]]
 * Explanation: A height of 2 is the maximum possible height of any assignment.
 * Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.
 *
 *  
 * Constraints:
 *
 * 	m == isWater.length
 * 	n == isWater[i].length
 * 	1 <= m, n <= 1000
 * 	isWater[i][j] is 0 or 1.
 * 	There is at least one water cell.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/map-of-highest-peak/
// discuss: https://leetcode.com/problems/map-of-highest-peak/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = is_water.len();
        let m = is_water[0].len();

        let mut result = vec![vec![1_000_000_000; m]; n];
        let mut stack = vec![];
        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    stack.push((i, j, 0));
                    result[i][j] = 0;
                }
            }
        }

        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((i, j, cv)) = stack.pop() {
                let nv = cv + 1;
                if 0 < i && result[i - 1][j] > nv {
                    result[i - 1][j] = nv;
                    new_stack.push((i - 1, j, nv));
                }

                if i < n - 1 && result[i + 1][j] > nv {
                    result[i + 1][j] = nv;
                    new_stack.push((i + 1, j, nv));
                }

                if 0 < j && result[i][j - 1] > nv {
                    result[i][j - 1] = nv;
                    new_stack.push((i, j - 1, nv));
                }

                if j < m - 1 && result[i][j + 1] > nv {
                    result[i][j + 1] = nv;
                    new_stack.push((i, j + 1, nv));
                }
            }
            stack = new_stack;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1765_example_1() {
        let is_water = vec![vec![0, 1], vec![0, 0]];

        let result = vec![vec![1, 0], vec![2, 1]];

        assert_eq!(Solution::highest_peak(is_water), result);
    }

    #[test]
    fn test_1765_example_2() {
        let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];

        let result = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]];

        assert_eq!(Solution::highest_peak(is_water), result);
    }
}
