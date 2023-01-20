/**
 * [0950] Reveal Cards In Increasing Order
 *
 * You are given an integer array deck. There is a deck of cards where every card has a unique integer. The integer on the i^th card is deck[i].
 * You can order the deck in any order you want. Initially, all the cards start face down (unrevealed) in one deck.
 * You will do the following steps repeatedly until all cards are revealed:
 * <ol>
 * 	Take the top card of the deck, reveal it, and take it out of the deck.
 * 	If there are still cards in the deck then put the next top card of the deck at the bottom of the deck.
 * 	If there are still unrevealed cards, go back to step 1. Otherwise, stop.
 * </ol>
 * Return an ordering of the deck that would reveal the cards in increasing order.
 * Note that the first entry in the answer is considered to be the top of the deck.
 *  
 * Example 1:
 *
 * Input: deck = [17,13,11,2,3,5,7]
 * Output: [2,13,3,11,5,17,7]
 * Explanation:
 * We get the deck in the order [17,13,11,2,3,5,7] (this order does not matter), and reorder it.
 * After reordering, the deck starts as [2,13,3,11,5,17,7], where 2 is the top of the deck.
 * We reveal 2, and move 13 to the bottom.  The deck is now [3,11,5,17,7,13].
 * We reveal 3, and move 11 to the bottom.  The deck is now [5,17,7,13,11].
 * We reveal 5, and move 17 to the bottom.  The deck is now [7,13,11,17].
 * We reveal 7, and move 13 to the bottom.  The deck is now [11,17,13].
 * We reveal 11, and move 17 to the bottom.  The deck is now [13,17].
 * We reveal 13, and move 17 to the bottom.  The deck is now [17].
 * We reveal 17.
 * Since all the cards revealed are in increasing order, the answer is correct.
 *
 * Example 2:
 *
 * Input: deck = [1,1000]
 * Output: [1,1000]
 *
 *  
 * Constraints:
 *
 * 	1 <= deck.length <= 1000
 * 	1 <= deck[i] <= 10^6
 * 	All the values of deck are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reveal-cards-in-increasing-order/
// discuss: https://leetcode.com/problems/reveal-cards-in-increasing-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let n = deck.len();
        let mut deck = deck;
        deck.sort();

        let (mut queue, mut result) = (std::collections::VecDeque::new(), vec![0; n]);
        for i in 0..n {
            queue.push_back(i);
        }

        for i in 0..n {
            if let Some(index) = queue.pop_front() {
                result[index] = deck[i];
            }

            if let Some(index) = queue.pop_front() {
                queue.push_back(index);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0950_example_1() {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let result = vec![2, 13, 3, 11, 5, 17, 7];

        assert_eq!(Solution::deck_revealed_increasing(deck), result);
    }

    #[test]
    fn test_0950_example_2() {
        let deck = vec![1, 1000];
        let result = vec![1, 1000];

        assert_eq!(Solution::deck_revealed_increasing(deck), result);
    }
}
