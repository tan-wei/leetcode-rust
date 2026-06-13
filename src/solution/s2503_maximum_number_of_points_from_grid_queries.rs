/**
 * [2503] Maximum Number of Points From Grid Queries
 *
 * You are given an m x n integer matrix grid and an array queries of size k.
 * Find an array answer of size k such that for each integer queries[i] you start in the top left cell of the matrix and repeat the following process:
 *
 * 	If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
 * 	Otherwise, you do not get any points, and you end this process.
 *
 * After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.
 * Return the resulting array answer.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2025/03/15/image1.png" style="width: 571px; height: 152px;" />
 * Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
 * Output: [5,8,1]
 * Explanation: The diagrams above show which cells we visit to get points for each query.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/20/yetgriddrawio-2.png" />
 * Input: grid = [[5,2,1],[1,1,2]], queries = [3]
 * Output: [0]
 * Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	2 <= m, n <= 1000
 * 	4 <= m * n <= 10^5
 * 	k == queries.length
 * 	1 <= k <= 10^4
 * 	1 <= grid[i][j], queries[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
// discuss: https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2503_example_1() {
        let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
        let queries = vec![5, 6, 2];

        let result = vec![5, 8, 1];

        assert_eq!(Solution::max_points(grid, queries), result);
    }

    #[test]
    #[ignore]
    fn test_2503_example_2() {
        let grid = vec![vec![5, 2, 1], vec![1, 1, 2]];
        let queries = vec![3];

        let result = vec![0];

        assert_eq!(Solution::max_points(grid, queries), result);
    }
}
