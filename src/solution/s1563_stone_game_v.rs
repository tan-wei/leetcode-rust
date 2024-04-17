/**
 * [1563] Stone Game V
 *
 * There are several stones arranged in a row, and each stone has an associated value which is an integer given in the array stoneValue.
 * In each round of the game, Alice divides the row into two non-empty rows (i.e. left row and right row), then Bob calculates the value of each row which is the sum of the values of all the stones in this row. Bob throws away the row which has the maximum value, and Alice's score increases by the value of the remaining row. If the value of the two rows are equal, Bob lets Alice decide which row will be thrown away. The next round starts with the remaining row.
 * The game ends when there is only one stone remaining. Alice's is initially zero.
 * Return the maximum score that Alice can obtain.
 *  
 * Example 1:
 *
 * Input: stoneValue = [6,2,3,4,5,5]
 * Output: 18
 * Explanation: In the first round, Alice divides the row to [6,2,3], [4,5,5]. The left row has the value 11 and the right row has value 14. Bob throws away the right row and Alice's score is now 11.
 * In the second round Alice divides the row to [6], [2,3]. This time Bob throws away the left row and Alice's score becomes 16 (11 + 5).
 * The last round Alice has only one choice to divide the row which is [2], [3]. Bob throws away the right row and Alice's score is now 18 (16 + 2). The game ends because only one stone is remaining in the row.
 *
 * Example 2:
 *
 * Input: stoneValue = [7,7,7,7,7,7,7]
 * Output: 28
 *
 * Example 3:
 *
 * Input: stoneValue = [4]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= stoneValue.length <= 500
 * 	1 <= stoneValue[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-v/
// discuss: https://leetcode.com/problems/stone-game-v/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/stone-game-v/solutions/3171103/just-a-runnable-solution/
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; stone_value.len()]; stone_value.len()];
        let mut sum = vec![0; stone_value.len() + 1];

        for i in 0..stone_value.len() {
            sum[i + 1] = sum[i] + stone_value[i];
        }

        for i in (0..stone_value.len()).rev() {
            for j in i + 1..stone_value.len() {
                for k in i..j {
                    let left = sum[k + 1] - sum[i];
                    let right = sum[j + 1] - sum[k + 1];
                    match left.cmp(&right) {
                        std::cmp::Ordering::Less => {
                            dp[i][j] = dp[i][j].max(dp[i][k] + left);
                        }
                        std::cmp::Ordering::Greater => {
                            dp[i][j] = dp[i][j].max(dp[k + 1][j] + right);
                        }
                        std::cmp::Ordering::Equal => {
                            dp[i][j] = dp[i][j].max(dp[i][k] + left);
                            dp[i][j] = dp[i][j].max(dp[k + 1][j] + right);
                        }
                    }
                }
            }
        }

        dp[0][stone_value.len() - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1563_example_1() {
        let stone_value = vec![6, 2, 3, 4, 5, 5];

        let result = 18;

        assert_eq!(Solution::stone_game_v(stone_value), result);
    }

    #[test]
    fn test_1563_example_2() {
        let stone_value = vec![7, 7, 7, 7, 7, 7, 7];

        let result = 28;

        assert_eq!(Solution::stone_game_v(stone_value), result);
    }

    #[test]
    fn test_1563_example_3() {
        let stone_value = vec![4];

        let result = 0;

        assert_eq!(Solution::stone_game_v(stone_value), result);
    }
}
