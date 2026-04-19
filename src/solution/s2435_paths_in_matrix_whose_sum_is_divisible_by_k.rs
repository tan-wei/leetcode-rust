/**
 * [2435] Paths in Matrix Whose Sum Is Divisible by K
 *
 * You are given a 0-indexed m x n integer matrix grid and an integer k. You are currently at position (0, 0) and you want to reach position (m - 1, n - 1) moving only down or right.
 * Return the number of paths where the sum of the elements on the path is divisible by k. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2022/08/13/image-20220813183124-1.png" style="width: 437px; height: 200px;" />
 * Input: grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
 * Output: 2
 * Explanation: There are two paths where the sum of the elements on the path is divisible by k.
 * The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
 * The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
 *
 * Example 2:
 * <img src="https://assets.leetcode.com/uploads/2022/08/17/image-20220817112930-3.png" style="height: 85px; width: 132px;" />
 * Input: grid = [[0,0]], k = 5
 * Output: 1
 * Explanation: The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
 *
 * Example 3:
 * <img src="https://assets.leetcode.com/uploads/2022/08/12/image-20220812224605-3.png" style="width: 257px; height: 200px;" />
 * Input: grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
 * Output: 10
 * Explanation: Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 5 * 10^4
 * 	1 <= m * n <= 5 * 10^4
 * 	0 <= grid[i][j] <= 100
 * 	1 <= k <= 50
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
// discuss: https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2435_example_1() {
        let grid = vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]];
        let k = 3;

        let result = 2;

        assert_eq!(Solution::number_of_paths(grid, k), result)
    }

    #[test]
    #[ignore]
    fn test_2435_example_2() {
        let grid = vec![vec![0, 0]];
        let k = 5;

        let result = 4;

        assert_eq!(Solution::number_of_paths(grid, k), result)
    }

    #[test]
    #[ignore]
    fn test_2435_example_3() {
        let grid = vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]];
        let k = 1;

        let result = 10;

        assert_eq!(Solution::number_of_paths(grid, k), result)
    }
}
