/**
 * [1289] Minimum Falling Path Sum II
 *
 * Given an n x n integer matrix grid, return the minimum sum of a falling path with non-zero shifts.
 * A falling path with non-zero shifts is a choice of exactly one element from each row of grid such that no two elements chosen in adjacent rows are in the same column.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/falling-grid.jpg" style="width: 244px; height: 245px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: 13
 * Explanation:
 * The possible falling paths are:
 * [1,5,9], [1,5,7], [1,6,7], [1,6,8],
 * [2,4,8], [2,4,9], [2,6,7], [2,6,8],
 * [3,4,8], [3,4,9], [3,5,7], [3,5,9]
 * The falling path with the smallest sum is [1,5,7], so the answer is 13.
 *
 * Example 2:
 *
 * Input: grid = [[7]]
 * Output: 7
 *
 *
 * Constraints:
 *
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 200
 * 	-99 <= grid[i][j] <= 99
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-falling-path-sum-ii/
// discuss: https://leetcode.com/problems/minimum-falling-path-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-falling-path-sum-ii/solutions/1980906/rust-solution/
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let inf = 1_000_000_000;
        let mut min = inf;
        let mut memo = vec![inf; m];

        for i in 0..m {
            memo[i] = std::cmp::min(memo[i], grid[0][i]);
        }

        for i in 1..n {
            let mut new_memo = vec![inf; m];
            for j in 0..m {
                let v = memo[j];
                for k in 0..m {
                    if j == k {
                        continue;
                    }
                    new_memo[k] = std::cmp::min(new_memo[k], v + grid[i][k]);
                }
            }
            memo = new_memo;
        }

        for v in memo {
            min = std::cmp::min(min, v);
        }

        min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1289_example_1() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = 13;

        assert_eq!(Solution::min_falling_path_sum(grid), result);
    }

    #[test]
    fn test_1289_example_2() {
        let grid = vec![vec![7]];
        let result = 7;

        assert_eq!(Solution::min_falling_path_sum(grid), result);
    }
}
