/**
 * [174] Dungeon Game
 *
 * The demons had captured the princess and imprisoned her in the bottom-right corner of a dungeon. The dungeon consists of m x n rooms laid out in a 2D grid. Our valiant knight was initially positioned in the top-left room and must fight his way through dungeon to rescue the princess.
 * The knight has an initial health point represented by a positive integer. If at any point his health point drops to 0 or below, he dies immediately.
 * Some of the rooms are guarded by demons (represented by negative integers), so the knight loses health upon entering these rooms; other rooms are either empty (represented as 0) or contain magic orbs that increase the knight's health (represented by positive integers).
 * To reach the princess as quickly as possible, the knight decides to move only rightward or downward in each step.
 * Return the knight's minimum initial health so that he can rescue the princess.
 * Note that any room can contain threats or power-ups, even the first room the knight enters and the bottom-right room where the princess is imprisoned.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/13/dungeon-grid-1.jpg" style="width: 253px; height: 253px;" />
 * Input: dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]]
 * Output: 7
 * Explanation: The initial health of the knight must be at least 7 if he follows the optimal path: RIGHT-> RIGHT -> DOWN -> DOWN.
 *
 * Example 2:
 *
 * Input: dungeon = [[0]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	m == dungeon.length
 * 	n == dungeon[i].length
 * 	1 <= m, n <= 200
 * 	-1000 <= dungeon[i][j] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/dungeon-game/
// discuss: https://leetcode.com/problems/dungeon-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/dungeon-game/discuss/698571/Rust-DP-bottom-up-0ms-explanation-by-example.
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![std::i32::MAX; n]; m];
        dp[m - 1][n - 1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i > 0 {
                    dp[i - 1][j] =
                        std::cmp::min(std::cmp::max(dp[i][j] - dungeon[i][j], 1), dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j - 1] =
                        std::cmp::min(std::cmp::max(dp[i][j] - dungeon[i][j], 1), dp[i][j - 1]);
                }
            }
        }
        std::cmp::max(dp[0][0] - dungeon[0][0], 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0174_example_1() {
        let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        let result = 7;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), result);
    }

    #[test]
    fn test_0174_example_2() {
        let dungeon = vec![vec![0]];
        let result = 1;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), result);
    }
}
