/**
 * [1686] Stone Game VI
 *
 * Alice and Bob take turns playing a game, with Alice starting first.
 * There are n stones in a pile. On each player's turn, they can remove a stone from the pile and receive points based on the stone's value. Alice and Bob may value the stones differently.
 * You are given two integer arrays of length n, aliceValues and bobValues. Each aliceValues[i] and bobValues[i] represents how Alice and Bob, respectively, value the i^th stone.
 * The winner is the person with the most points after all the stones are chosen. If both players have the same amount of points, the game results in a draw. Both players will play optimally. Both players know the other's values.
 * Determine the result of the game, and:
 *
 * 	If Alice wins, return 1.
 * 	If Bob wins, return -1.
 * 	If the game results in a draw, return 0.
 *
 *  
 * Example 1:
 *
 * Input: aliceValues = [1,3], bobValues = [2,1]
 * Output: 1
 * Explanation:
 * If Alice takes stone 1 (0-indexed) first, Alice will receive 3 points.
 * Bob can only choose stone 0, and will only receive 2 points.
 * Alice wins.
 *
 * Example 2:
 *
 * Input: aliceValues = [1,2], bobValues = [3,1]
 * Output: 0
 * Explanation:
 * If Alice takes stone 0, and Bob takes stone 1, they will both have 1 point.
 * Draw.
 *
 * Example 3:
 *
 * Input: aliceValues = [2,4,3], bobValues = [1,6,7]
 * Output: -1
 * Explanation:
 * Regardless of how Alice plays, Bob will be able to have more points than Alice.
 * For example, if Alice takes stone 1, Bob can take stone 2, and Alice takes stone 0, Alice will have 6 points to Bob's 7.
 * Bob wins.
 *
 *  
 * Constraints:
 *
 * 	n == aliceValues.length == bobValues.length
 * 	1 <= n <= 10^5
 * 	1 <= aliceValues[i], bobValues[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-vi/
// discuss: https://leetcode.com/problems/stone-game-vi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let n = alice_values.len();
        let mut memo = vec![(0, 0); n];

        for i in 0..n {
            memo[i] = (alice_values[i] + bob_values[i], i);
        }

        memo.sort_unstable();
        memo.reverse();

        let mut a = 0;
        let mut b = 0;

        for i in 0..n {
            if i % 2 == 0 {
                a += alice_values[memo[i].1];
            } else {
                b += bob_values[memo[i].1];
            }
        }

        match a.cmp(&b) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1686_example_1() {
        let alice_values = vec![1, 3];
        let bob_values = vec![2, 1];

        let result = 1;

        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), result);
    }

    #[test]
    fn test_1686_example_2() {
        let alice_values = vec![1, 2];
        let bob_values = vec![3, 1];

        let result = 0;

        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), result);
    }

    #[test]
    fn test_1686_example_3() {
        let alice_values = vec![2, 4, 3];
        let bob_values = vec![1, 6, 7];

        let result = -1;

        assert_eq!(Solution::stone_game_vi(alice_values, bob_values), result);
    }
}
