/**
 * [0764] Largest Plus Sign
 *
 * You are given an integer n. You have an n x n binary grid grid with all values initially 1's except for some indices given in the array mines. The i^th element of the array mines is defined as mines[i] = [xi, yi] where grid[xi][yi] == 0.
 * Return the order of the largest axis-aligned plus sign of 1's contained in grid. If there is none, return 0.
 * An axis-aligned plus sign of 1's of order k has some center grid[r][c] == 1 along with four arms of length k - 1 going up, down, left, and right, and made of 1's. Note that there could be 0's or 1's beyond the arms of the plus sign, only the relevant area of the plus sign is checked for 1's.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/plus1-grid.jpg" style="width: 404px; height: 405px;" />
 * Input: n = 5, mines = [[4,2]]
 * Output: 2
 * Explanation: In the above grid, the largest plus sign can only be of order 2. One of them is shown.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/plus2-grid.jpg" style="width: 84px; height: 85px;" />
 * Input: n = 1, mines = [[0,0]]
 * Output: 0
 * Explanation: There is no plus sign, so return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 500
 * 	1 <= mines.length <= 5000
 * 	0 <= xi, yi < n
 * 	All the pairs (xi, yi) are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-plus-sign/
// discuss: https://leetcode.com/problems/largest-plus-sign/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut banned = std::collections::HashSet::<usize>::new();
        for mine in &mines {
            banned.insert((mine[0] * n + mine[1]) as usize);
        }
        let mut dp = vec![vec![0; n as usize]; n as usize];
        let mut result = 0;
        let mut count = 0;

        for r in 0..n as usize {
            count = 0;
            for c in 0..n as usize {
                count = if banned.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count;
            }
            count = 0;
            for c in (0..n as usize).rev() {
                count = if banned.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = std::cmp::min(count, dp[r][c]);
            }
        }

        for c in 0..n as usize {
            count = 0;
            for r in 0..n as usize {
                count = if banned.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = std::cmp::min(count, dp[r][c]);
            }
            count = 0;
            for r in (0..n as usize).rev() {
                count = if banned.contains(&(r * n as usize + c)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = std::cmp::min(count, dp[r][c]);
                result = std::cmp::max(result, dp[r][c]);
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
    fn test_0764_example_1() {
        let n = 5;
        let mines = vec![vec![4, 2]];
        let result = 2;

        assert_eq!(Solution::order_of_largest_plus_sign(n, mines), result);
    }

    #[test]
    fn test_0764_example_2() {
        let n = 1;
        let mines = vec![vec![0, 0]];
        let result = 0;

        assert_eq!(Solution::order_of_largest_plus_sign(n, mines), result);
    }
}
