/**
 * [1931] Painting a Grid With Three Different Colors
 *
 * You are given two integers m and n. Consider an m x n grid where each cell is initially white. You can paint each cell red, green, or blue. All cells must be painted.
 * Return the number of ways to color the grid with no two adjacent cells having the same color. Since the answer can be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/22/colorthegrid.png" style="width: 200px; height: 50px;" />
 * Input: m = 1, n = 1
 * Output: 3
 * Explanation: The three possible colorings are shown in the image above.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/22/copy-of-colorthegrid.png" style="width: 321px; height: 121px;" />
 * Input: m = 1, n = 2
 * Output: 6
 * Explanation: The six possible colorings are shown in the image above.
 *
 * Example 3:
 *
 * Input: m = 5, n = 5
 * Output: 580986
 *
 *  
 * Constraints:
 *
 * 	1 <= m <= 5
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/painting-a-grid-with-three-different-colors/
// discuss: https://leetcode.com/problems/painting-a-grid-with-three-different-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1931_example_1() {
        let m = 1;
        let n = 1;

        let result = 3;

        assert_eq!(Solution::color_the_grid(m, n), result);
    }

    #[test]
    #[ignore]
    fn test_1931_example_2() {
        let m = 1;
        let n = 2;

        let result = 6;

        assert_eq!(Solution::color_the_grid(m, n), result);
    }

    #[test]
    #[ignore]
    fn test_1931_example_3() {
        let m = 5;
        let n = 5;

        let result = 580986;

        assert_eq!(Solution::color_the_grid(m, n), result);
    }
}
