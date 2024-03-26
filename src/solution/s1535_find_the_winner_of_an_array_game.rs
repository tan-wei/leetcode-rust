/**
 * [1535] Find the Winner of an Array Game
 *
 * Given an integer array arr of distinct integers and an integer k.
 * A game will be played between the first two elements of the array (i.e. arr[0] and arr[1]). In each round of the game, we compare arr[0] with arr[1], the larger integer wins and remains at position 0, and the smaller integer moves to the end of the array. The game ends when an integer wins k consecutive rounds.
 * Return the integer which will win the game.
 * It is guaranteed that there will be a winner of the game.
 *  
 * Example 1:
 *
 * Input: arr = [2,1,3,5,4,6,7], k = 2
 * Output: 5
 * Explanation: Let's see the rounds of the game:
 * Round |       arr       | winner | win_count
 *   1   | [2,1,3,5,4,6,7] | 2      | 1
 *   2   | [2,3,5,4,6,7,1] | 3      | 1
 *   3   | [3,5,4,6,7,1,2] | 5      | 1
 *   4   | [5,4,6,7,1,2,3] | 5      | 2
 * So we can see that 4 rounds will be played and 5 is the winner because it wins 2 consecutive games.
 *
 * Example 2:
 *
 * Input: arr = [3,2,1], k = 10
 * Output: 3
 * Explanation: 3 will win the first 10 rounds consecutively.
 *
 *  
 * Constraints:
 *
 * 	2 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 10^6
 * 	arr contains distinct integers.
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-winner-of-an-array-game/
// discuss: https://leetcode.com/problems/find-the-winner-of-an-array-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut result = arr[0];
        if k == 1 {
            result.max(arr[1])
        } else {
            arr.iter().try_fold(-1, |c, &x| {
                if x > result {
                    result = x;
                    Some(1)
                } else {
                    if c + 1 == k {
                        None
                    } else {
                        Some(c + 1)
                    }
                }
            });
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1535_example_1() {
        let arr = vec![2, 1, 3, 5, 4, 6, 7];
        let k = 2;

        let result = 5;

        assert_eq!(Solution::get_winner(arr, k), result);
    }

    #[test]
    fn test_1535_example_2() {
        let arr = vec![3, 2, 1];
        let k = 10;

        let result = 3;

        assert_eq!(Solution::get_winner(arr, k), result);
    }
}
