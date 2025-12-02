/**
 * [2260] Minimum Consecutive Cards to Pick Up
 *
 * You are given an integer array cards where cards[i] represents the value of the i^th card. A pair of cards are matching if the cards have the same value.
 * Return the minimum number of consecutive cards you have to pick up to have a pair of matching cards among the picked cards. If it is impossible to have matching cards, return -1.
 *  
 * Example 1:
 *
 * Input: cards = [3,4,2,3,4,7]
 * Output: 4
 * Explanation: We can pick up the cards [3,4,2,3] which contain a matching pair of cards with value 3. Note that picking up the cards [4,2,3,4] is also optimal.
 *
 * Example 2:
 *
 * Input: cards = [1,0,5,3]
 * Output: -1
 * Explanation: There is no way to pick up a set of consecutive cards that contain a pair of matching cards.
 *
 *  
 * Constraints:
 *
 * 	1 <= cards.length <= 10^5
 * 	0 <= cards[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/
// discuss: https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2260_example_1() {
        let cards = vec![3, 4, 2, 3, 4, 7];

        let result = 4;

        assert_eq!(Solution::minimum_card_pickup(cards), result);
    }

    #[test]
    #[ignore]
    fn test_2260_example_2() {
        let cards = vec![1, 0, 5, 3];

        let result = -1;

        assert_eq!(Solution::minimum_card_pickup(cards), result);
    }
}
