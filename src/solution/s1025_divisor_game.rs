/**
 * [1025] Divisor Game
 *
 * Alice and Bob take turns playing a game, with Alice starting first.
 * Initially, there is a number n on the chalkboard. On each player's turn, that player makes a move consisting of:
 *
 * 	Choosing any x with 0 < x < n and n % x == 0.
 * 	Replacing the number n on the chalkboard with n - x.
 *
 * Also, if a player cannot make a move, they lose the game.
 * Return true if and only if Alice wins the game, assuming both players play optimally.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: true
 * Explanation: Alice chooses 1, and Bob has no more moves.
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: false
 * Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divisor-game/
// discuss: https://leetcode.com/problems/divisor-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1025_example_1() {
        let n = 2;
        let result = true;

        assert_eq!(Solution::divisor_game(n), result);
    }

    #[test]
    fn test_1025_example_2() {
        let n = 3;
        let result = false;

        assert_eq!(Solution::divisor_game(n), result);
    }
}
