/**
 * [1753] Maximum Score From Removing Stones
 *
 * You are playing a solitaire game with three piles of stones of sizes a​​​​​​, b,​​​​​​ and c​​​​​​ respectively. Each turn you choose two different non-empty piles, take one stone from each, and add 1 point to your score. The game stops when there are fewer than two non-empty piles (meaning there are no more available moves).
 * Given three integers a​​​​​, b,​​​​​ and c​​​​​, return the maximum score you can get.
 *  
 * Example 1:
 *
 * Input: a = 2, b = 4, c = 6
 * Output: 6
 * Explanation: The starting state is (2, 4, 6). One optimal set of moves is:
 * - Take from 1st and 3rd piles, state is now (1, 4, 5)
 * - Take from 1st and 3rd piles, state is now (0, 4, 4)
 * - Take from 2nd and 3rd piles, state is now (0, 3, 3)
 * - Take from 2nd and 3rd piles, state is now (0, 2, 2)
 * - Take from 2nd and 3rd piles, state is now (0, 1, 1)
 * - Take from 2nd and 3rd piles, state is now (0, 0, 0)
 * There are fewer than two non-empty piles, so the game ends. Total: 6 points.
 *
 * Example 2:
 *
 * Input: a = 4, b = 4, c = 6
 * Output: 7
 * Explanation: The starting state is (4, 4, 6). One optimal set of moves is:
 * - Take from 1st and 2nd piles, state is now (3, 3, 6)
 * - Take from 1st and 3rd piles, state is now (2, 3, 5)
 * - Take from 1st and 3rd piles, state is now (1, 3, 4)
 * - Take from 1st and 3rd piles, state is now (0, 3, 3)
 * - Take from 2nd and 3rd piles, state is now (0, 2, 2)
 * - Take from 2nd and 3rd piles, state is now (0, 1, 1)
 * - Take from 2nd and 3rd piles, state is now (0, 0, 0)
 * There are fewer than two non-empty piles, so the game ends. Total: 7 points.
 *
 * Example 3:
 *
 * Input: a = 1, b = 8, c = 8
 * Output: 8
 * Explanation: One optimal set of moves is to take from the 2nd and 3rd piles for 8 turns until they are empty.
 * After that, there are fewer than two non-empty piles, so the game ends.
 *
 *  
 * Constraints:
 *
 * 	1 <= a, b, c <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-from-removing-stones/
// discuss: https://leetcode.com/problems/maximum-score-from-removing-stones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut piles = vec![a, b, c];
        piles.sort_unstable();
        let max_score = std::cmp::min(piles[0] + piles[1], (piles[0] + piles[1] + piles[2]) / 2);
        max_score
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1753_example_1() {
        let a = 2;
        let b = 4;
        let c = 6;

        let result = 6;

        assert_eq!(Solution::maximum_score(a, b, c), result);
    }

    #[test]
    fn test_1753_example_2() {
        let a = 4;
        let b = 4;
        let c = 6;

        let result = 7;

        assert_eq!(Solution::maximum_score(a, b, c), result);
    }

    #[test]
    fn test_1753_example_3() {
        let a = 1;
        let b = 8;
        let c = 8;

        let result = 8;

        assert_eq!(Solution::maximum_score(a, b, c), result);
    }
}
