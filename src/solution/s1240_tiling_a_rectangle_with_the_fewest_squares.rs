/**
 * [1240] Tiling a Rectangle with the Fewest Squares
 *
 * Given a rectangle of size n x m, return the minimum number of integer-sided squares that tile the rectangle.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_11_1592.png" style="width: 154px; height: 106px;" />
 *
 * Input: n = 2, m = 3
 * Output: 3
 * Explanation: 3 squares are necessary to cover the rectangle.
 * 2 (squares of 1x1)
 * 1 (square of 2x2)
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_22_1592.png" style="width: 224px; height: 126px;" />
 *
 * Input: n = 5, m = 8
 * Output: 5
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_33_1592.png" style="width: 224px; height: 189px;" />
 *
 * Input: n = 11, m = 13
 * Output: 6
 *
 *  
 * Constraints:
 *
 * 	1 <= n, m <= 13
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/
// discuss: https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/solutions/779495/rust-translated-0ms-2-1m/
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        if m < n {
            std::mem::swap(&mut n, &mut m);
        }
        let mut memo = vec![vec![-1; m as usize + 1]; n as usize + 1];
        Self::dfs(n as usize, m as usize, &mut memo)
    }

    fn dfs(mut n: usize, mut m: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if m < n {
            std::mem::swap(&mut n, &mut m);
        }
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return m as i32;
        }
        if memo[n as usize][m as usize] != -1 {
            return memo[n as usize][m as usize];
        };
        let mut result = std::i32::MAX;
        for i in 1..=n {
            let nn = n - i;
            let mm = m - i;
            result = std::cmp::min(
                result,
                std::cmp::min(
                    Self::dfs(nn, m, memo) + Self::dfs(i, mm, memo),
                    Self::dfs(nn, i, memo) + Self::dfs(mm, n, memo),
                ) + 1,
            );

            for j in 0..std::cmp::min(nn, i) {
                result = std::cmp::min(
                    result,
                    Self::dfs(nn, i - j, memo)
                        + Self::dfs(nn - j, mm + j, memo)
                        + Self::dfs(mm, i + j, memo)
                        + 2,
                );
            }

            for j in 1..std::cmp::min(nn, i) {
                result = std::cmp::min(
                    result,
                    Self::dfs(mm, i - j, memo)
                        + Self::dfs(mm - j, nn + j, memo)
                        + Self::dfs(nn, i + j, memo)
                        + 2,
                );
            }
        }
        memo[n as usize][m as usize] = result;
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1240_example_1() {
        let n = 2;
        let m = 3;
        let result = 3;

        assert_eq!(Solution::tiling_rectangle(n, m), result);
    }

    #[test]
    fn test_1240_example_2() {
        let n = 5;
        let m = 7;
        let result = 5;

        assert_eq!(Solution::tiling_rectangle(n, m), result);
    }

    #[test]
    fn test_1240_example_3() {
        let n = 11;
        let m = 13;
        let result = 6;

        assert_eq!(Solution::tiling_rectangle(n, m), result);
    }
}
