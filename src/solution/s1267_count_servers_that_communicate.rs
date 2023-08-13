/**
 * [1267] Count Servers that Communicate
 *
 * You are given a map of a server center, represented as a m * n integer matrix grid, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.<br />
 * <br />
 * Return the number of servers that communicate with any other server.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-6.jpg" style="width: 202px; height: 203px;" />
 *
 * Input: grid = [[1,0],[0,1]]
 * Output: 0
 * Explanation: No servers can communicate with others.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/13/untitled-diagram-4.jpg" style="width: 203px; height: 203px;" />
 *
 * Input: grid = [[1,0],[1,1]]
 * Output: 3
 * Explanation: All three servers can communicate with at least one other server.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-1-3.jpg" style="width: 443px; height: 443px;" />
 *
 * Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
 * Output: 4
 * Explanation: The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can't communicate with any other server.
 *
 *
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m <= 250
 * 	1 <= n <= 250
 * 	grid[i][j] == 0 or 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-servers-that-communicate/
// discuss: https://leetcode.com/problems/count-servers-that-communicate/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n: usize = grid.len();
        let m: usize = grid[0].len();
        let mut row = vec![0; n];
        let mut col = vec![0; m];

        let mut result: i32 = 0;

        //count number of servers in each row and column
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        //count number of servers not alone in either row or column
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && (row[i] > 1 || col[j] > 1) {
                    result += 1;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1267_example_1() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let result = 0;

        assert_eq!(Solution::count_servers(grid), result);
    }

    #[test]
    fn test_1267_example_2() {
        let grid = vec![vec![1, 0], vec![1, 1]];
        let result = 3;

        assert_eq!(Solution::count_servers(grid), result);
    }

    #[test]
    fn test_1267_example_3() {
        let grid = vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let result = 4;

        assert_eq!(Solution::count_servers(grid), result);
    }
}
