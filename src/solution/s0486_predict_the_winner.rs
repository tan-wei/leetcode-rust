/**
 * [0486] Predict the Winner
 *
 * You are given an integer array nums. Two players are playing a game with this array: player 1 and player 2.
 * Player 1 and player 2 take turns, with player 1 starting first. Both players start the game with a score of 0. At each turn, the player takes one of the numbers from either end of the array (i.e., nums[0] or nums[nums.length - 1]) which reduces the size of the array by 1. The player adds the chosen number to their score. The game ends when there are no more elements in the array.
 * Return true if Player 1 can win the game. If the scores of both players are equal, then player 1 is still the winner, and you should also return true. You may assume that both players are playing optimally.
 *  
 * Example 1:
 *
 * Input: nums = [1,5,2]
 * Output: false
 * Explanation: Initially, player 1 can choose between 1 and 2.
 * If he chooses 2 (or 1), then player 2 can choose from 1 (or 2) and 5. If player 2 chooses 5, then player 1 will be left with 1 (or 2).
 * So, final score of player 1 is 1 + 2 = 3, and player 2 is 5.
 * Hence, player 1 will never be the winner and you need to return false.
 *
 * Example 2:
 *
 * Input: nums = [1,5,233,7]
 * Output: true
 * Explanation: Player 1 first chooses 1. Then player 2 has to choose between 5 and 7. No matter which number player 2 choose, player 1 can choose 233.
 * Finally, player 1 has more score (234) than player 2 (12), so you need to return True representing player1 can win.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 20
 * 	0 <= nums[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/predict-the-winner/
// discuss: https://leetcode.com/problems/predict-the-winner/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/predict-the-winner/discuss/811425/Rust-cheapest-and-best
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut dp = vec![vec![(0, 0); nums.len() + 1]];
        for i in 1..=nums.len() {
            dp.push(vec![]);
            for j in 0..=nums.len() - i {
                let pick_left = dp[i - 1][j + 1];
                let pick_right = dp[i - 1][j];
                dp[i].push(
                    if nums[j] + pick_left.1 - pick_left.0
                        > nums[j + i - 1] + pick_right.1 - pick_right.0
                    {
                        (nums[j] + pick_left.1, pick_left.0)
                    } else {
                        (nums[j + i - 1] + pick_right.1, pick_right.0)
                    },
                )
            }
        }

        let (p1, p2) = dp.last().unwrap()[0];
        p1 >= p2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0486_example_1() {
        let nums = vec![1, 5, 2];
        let result = false;

        assert_eq!(Solution::predict_the_winner(nums), result);
    }

    #[test]
    fn test_0486_example_2() {
        let nums = vec![1, 5, 233, 7];
        let result = true;

        assert_eq!(Solution::predict_the_winner(nums), result);
    }
}
