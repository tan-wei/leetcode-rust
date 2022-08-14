/**
 * [0790] Domino and Tromino Tiling
 *
 * You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/15/lc-domino.jpg" style="width: 362px; height: 195px;" />
 * Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 10^9 + 7.
 * In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/15/lc-domino1.jpg" style="width: 500px; height: 226px;" />
 * Input: n = 3
 * Output: 5
 * Explanation: The five different ways are show above.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/domino-and-tromino-tiling/
// discuss: https://leetcode.com/problems/domino-and-tromino-tiling/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: usize = 1_000_000_007;

        std::iter::successors(Some((1, 1, 0)), |&(a, b, c)| {
            Some((b, (a + b + 2 * c) % MOD, (c + a) % MOD))
        })
        .nth(n as usize - 1)
        .unwrap()
        .1 as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0790_example_1() {
        let n = 3;
        let result = 5;

        assert_eq!(Solution::num_tilings(n), result);
    }

    #[test]
    fn test_0790_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::num_tilings(n), result);
    }
}
