/**
 * [1631] Path With Minimum Effort
 *
 * You are a hiker preparing for an upcoming hike. You are given heights, a 2D array of size rows x columns, where heights[row][col] represents the height of cell (row, col). You are situated in the top-left cell, (0, 0), and you hope to travel to the bottom-right cell, (rows-1, columns-1) (i.e., 0-indexed). You can move up, down, left, or right, and you wish to find a route that requires the minimum effort.
 * A route's effort is the maximum absolute difference in heights between two consecutive cells of the route.
 * Return the minimum effort required to travel from the top-left cell to the bottom-right cell.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex1.png" style="width: 300px; height: 300px;" />
 *
 * Input: heights = [[1,2,2],[3,8,2],[5,3,5]]
 * Output: 2
 * Explanation: The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
 * This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex2.png" style="width: 300px; height: 300px;" />
 *
 * Input: heights = [[1,2,3],[3,8,4],[5,3,5]]
 * Output: 1
 * Explanation: The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex3.png" style="width: 300px; height: 300px;" />
 * Input: heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
 * Output: 0
 * Explanation: This route does not require any effort.
 *
 *  
 * Constraints:
 *
 * 	rows == heights.length
 * 	columns == heights[i].length
 * 	1 <= rows, columns <= 100
 * 	1 <= heights[i][j] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/path-with-minimum-effort/
// discuss: https://leetcode.com/problems/path-with-minimum-effort/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        let m = heights[0].len();

        let mut dp = vec![vec![i32::MAX; m]; n];
        dp[0][0] = 0;

        let mut bh = std::collections::BinaryHeap::<std::cmp::Reverse<(i32, usize, usize)>>::new();

        bh.push(std::cmp::Reverse((0, 0, 0)));

        while let Some(std::cmp::Reverse((_, i, j))) = bh.pop() {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(di, dj)| {
                    let to_ = ((i as i32 + di) as usize, (j as i32 + dj) as usize);

                    let to = *heights.get(to_.0)?.get(to_.1)?;

                    let from: i32 = heights[i][j];

                    let d = dp[i][j].max((from - to).abs());

                    if dp[to_.0][to_.1] > d {
                        Some((d, to_.0, to_.1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(d, i, j)| {
                    dp[i][j] = d;

                    bh.push(std::cmp::Reverse((d, i, j)));
                })
        }

        dp[n - 1][m - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1631_example_1() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];

        let result = 2;

        assert_eq!(Solution::minimum_effort_path(heights), result);
    }

    #[test]
    fn test_1631_example_2() {
        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];

        let result = 1;

        assert_eq!(Solution::minimum_effort_path(heights), result);
    }

    #[test]
    fn test_1631_example_3() {
        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];

        let result = 0;

        assert_eq!(Solution::minimum_effort_path(heights), result);
    }
}
