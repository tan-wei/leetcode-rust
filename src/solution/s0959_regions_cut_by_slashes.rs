/**
 * [0959] Regions Cut By Slashes
 *
 * An n x n grid is composed of 1 x 1 squares where each 1 x 1 square consists of a '/', '\', or blank space ' '. These characters divide the square into contiguous regions.
 * Given the grid grid represented as a string array, return the number of regions.
 * Note that backslash characters are escaped, so a '\' is represented as '\\'.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/1.png" style="width: 200px; height: 200px;" />
 * Input: grid = [" /","/ "]
 * Output: 2
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/2.png" style="width: 200px; height: 198px;" />
 * Input: grid = [" /","  "]
 * Output: 1
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/4.png" style="width: 200px; height: 200px;" />
 * Input: grid = ["/\\","\\/"]
 * Output: 5
 * Explanation: Recall that because \ characters are escaped, "\\/" refers to \/, and "/\\" refers to /\.
 *
 *  
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 30
 * 	grid[i][j] is either '/', '\', or ' '.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regions-cut-by-slashes/
// discuss: https://leetcode.com/problems/regions-cut-by-slashes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut graph = vec![];

        for g in grid {
            let mut v = vec![vec![]; 3];

            for c in g.chars() {
                let mut data = vec![vec![0; 3]; 3];
                if c == '/' {
                    for i in 0..3 {
                        data[i][2 - i] = 1;
                    }
                }
                if c == '\\' {
                    for i in 0..3 {
                        data[i][i] = 1;
                    }
                }

                for i in 0..3 {
                    for j in 0..3 {
                        v[i].push(data[i][j]);
                    }
                }
            }
            for it in v {
                graph.push(it);
            }
        }

        Self::count(&mut graph)
    }

    fn count(graph: &mut Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut ret = 0;

        for i in 0..n {
            for j in 0..n {
                if graph[i][j] == 1 {
                    continue;
                }
                Self::dfs_helper(graph, i, j);
                ret += 1;
            }
        }

        ret
    }

    fn dfs_helper(graph: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if graph[i][j] == 1 {
            return;
        }

        graph[i][j] = 1;
        if i > 0 {
            Self::dfs_helper(graph, i - 1, j);
        }
        if i + 1 < graph.len() {
            Self::dfs_helper(graph, i + 1, j);
        }
        if j > 0 {
            Self::dfs_helper(graph, i, j - 1);
        }
        if j + 1 < graph.len() {
            Self::dfs_helper(graph, i, j + 1);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0959_example_1() {
        let grid = vec_string![" /", "/ "];
        let result = 2;

        assert_eq!(Solution::regions_by_slashes(grid), result);
    }

    #[test]
    fn test_0959_example_2() {
        let grid = vec_string![" /", "  "];
        let result = 1;

        assert_eq!(Solution::regions_by_slashes(grid), result);
    }

    #[test]
    fn test_0959_example_3() {
        let grid = vec_string!["/\\", "\\/"];
        let result = 5;

        assert_eq!(Solution::regions_by_slashes(grid), result);
    }
}
