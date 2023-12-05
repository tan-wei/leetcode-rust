/**
 * [1406] Stone Game III
 *
 * Alice and Bob continue their games with piles of stones. There are several stones arranged in a row, and each stone has an associated value which is an integer given in the array stoneValue.
 * Alice and Bob take turns, with Alice starting first. On each player's turn, that player can take 1, 2, or 3 stones from the first remaining stones in the row.
 * The score of each player is the sum of the values of the stones taken. The score of each player is 0 initially.
 * The objective of the game is to end with the highest score, and the winner is the player with the highest score and there could be a tie. The game continues until all the stones have been taken.
 * Assume Alice and Bob play optimally.
 * Return "Alice" if Alice will win, "Bob" if Bob will win, or "Tie" if they will end the game with the same score.
 *  
 * Example 1:
 *
 * Input: stoneValue = [1,2,3,7]
 * Output: "Bob"
 * Explanation: Alice will always lose. Her best move will be to take three piles and the score become 6. Now the score of Bob is 7 and Bob wins.
 *
 * Example 2:
 *
 * Input: stoneValue = [1,2,3,-9]
 * Output: "Alice"
 * Explanation: Alice must choose all the three piles at the first move to win and leave Bob with negative score.
 * If Alice chooses one pile her score will be 1 and the next move Bob's score becomes 5. In the next move, Alice will take the pile with value = -9 and lose.
 * If Alice chooses two piles her score will be 3 and the next move Bob's score becomes 3. In the next move, Alice will take the pile with value = -9 and also lose.
 * Remember that both play optimally so here Alice will choose the scenario that makes her win.
 *
 * Example 3:
 *
 * Input: stoneValue = [1,2,3,6]
 * Output: "Tie"
 * Explanation: Alice cannot win this game. She can end the game in a draw if she decided to choose all the first three piles, otherwise she will lose.
 *
 *  
 * Constraints:
 *
 * 	1 <= stoneValue.length <= 5 * 10^4
 * 	-1000 <= stoneValue[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-iii/
// discuss: https://leetcode.com/problems/stone-game-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/stone-game-iii/solutions/3566343/rust-optimized-iterators-beats-100/
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();

        let alice_minus_bob = (0..n).rev().fold([0; 3], |net_scores, i| {
            let new_net_score = (0..3)
                .map(|j| stone_value[i..].iter().take(j + 1).sum::<i32>() - net_scores[j])
                .max()
                .unwrap();

            [new_net_score, net_scores[0], net_scores[1]]
        })[0];

        match alice_minus_bob {
            0 => "Tie",
            x if x > 0 => "Alice",
            _ => "Bob",
        }
        .into()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1406_example_1() {
        let stone_value = vec![1, 2, 3, 7];

        let result = "Bob".to_string();

        assert_eq!(Solution::stone_game_iii(stone_value), result);
    }

    #[test]
    fn test_1406_example_2() {
        let stone_value = vec![1, 2, 3, -9];

        let result = "Alice".to_string();

        assert_eq!(Solution::stone_game_iii(stone_value), result);
    }

    #[test]
    fn test_1406_example_3() {
        let stone_value = vec![1, 2, 3, 6];

        let result = "Tie".to_string();

        assert_eq!(Solution::stone_game_iii(stone_value), result);
    }
}
