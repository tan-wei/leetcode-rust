/**
 * [0822] Card Flipping Game
 *
 * You are given two 0-indexed integer arrays fronts and backs of length n, where the i^th card has the positive integer fronts[i] printed on the front and backs[i] printed on the back. Initially, each card is placed on a table such that the front number is facing up and the other is facing down. You may flip over any number of cards (possibly zero).
 * After flipping the cards, an integer is considered good if it is facing down on some card and not facing up on any card.
 * Return the minimum possible good integer after flipping the cards. If there are no good integers, return 0.
 *  
 * Example 1:
 *
 * Input: fronts = [1,2,4,4,7], backs = [1,3,4,1,3]
 * Output: 2
 * Explanation:
 * If we flip the second card, the face up numbers are [1,3,4,4,7] and the face down are [1,2,4,1,3].
 * 2 is the minimum good integer as it appears facing down but not facing up.
 * It can be shown that 2 is the minimum possible good integer obtainable after flipping some cards.
 *
 * Example 2:
 *
 * Input: fronts = [1], backs = [1]
 * Output: 0
 * Explanation:
 * There are no good integers no matter how we flip the cards, so we return 0.
 *
 *  
 * Constraints:
 *
 * 	n == fronts.length == backs.length
 * 	1 <= n <= 1000
 * 	1 <= fronts[i], backs[i] <= 2000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/card-flipping-game/
// discuss: https://leetcode.com/problems/card-flipping-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/card-flipping-game/discuss/309972/Rust-solution-30-lines-4ms-2.4-MB
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut bad = std::collections::HashSet::new();

        for (i, front) in fronts.iter().cloned().enumerate() {
            if front == backs[i] {
                bad.insert(front);
            }
        }

        let mut min_good = None::<i32>;
        for (i, front) in fronts.iter().cloned().enumerate() {
            if !bad.contains(&front) {
                min_good = Some(min_good.map_or(front, |min| std::cmp::min(min, front)));
            }

            let back = backs[i];
            if !bad.contains(&back) {
                min_good = Some(min_good.map_or(back, |min| std::cmp::min(min, back)));
            }
        }

        min_good.unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0822_example_1() {
        let fronts = vec![1, 2, 4, 4, 7];
        let backs = vec![1, 3, 4, 1, 3];
        let result = 2;

        assert_eq!(Solution::flipgame(fronts, backs), result);
    }

    #[test]
    fn test_0822_example_2() {
        let fronts = vec![1];
        let backs = vec![1];
        let result = 0;

        assert_eq!(Solution::flipgame(fronts, backs), result);
    }
}
