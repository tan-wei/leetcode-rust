/**
 * [2347] Best Poker Hand
 *
 * You are given an integer array ranks and a character array suits. You have 5 cards where the i^th card has a rank of ranks[i] and a suit of suits[i].
 * The following are the types of poker hands you can make from best to worst:
 * <ol>
 * 	"Flush": Five cards of the same suit.
 * 	"Three of a Kind": Three cards of the same rank.
 * 	"Pair": Two cards of the same rank.
 * 	"High Card": Any single card.
 * </ol>
 * Return a string representing the best type of poker hand you can make with the given cards.
 * Note that the return values are case-sensitive.
 *  
 * Example 1:
 *
 * Input: ranks = [13,2,3,1,9], suits = ["a","a","a","a","a"]
 * Output: "Flush"
 * Explanation: The hand with all the cards consists of 5 cards with the same suit, so we have a "Flush".
 *
 * Example 2:
 *
 * Input: ranks = [4,4,2,4,4], suits = ["d","a","a","b","c"]
 * Output: "Three of a Kind"
 * Explanation: The hand with the first, second, and fourth card consists of 3 cards with the same rank, so we have a "Three of a Kind".
 * Note that we could also make a "Pair" hand but "Three of a Kind" is a better hand.
 * Also note that other cards could be used to make the "Three of a Kind" hand.
 * Example 3:
 *
 * Input: ranks = [10,10,2,12,9], suits = ["a","b","c","a","d"]
 * Output: "Pair"
 * Explanation: The hand with the first and second card consists of 2 cards with the same rank, so we have a "Pair".
 * Note that we cannot make a "Flush" or a "Three of a Kind".
 *
 *  
 * Constraints:
 *
 * 	ranks.length == suits.length == 5
 * 	1 <= ranks[i] <= 13
 * 	'a' <= suits[i] <= 'd'
 * 	No two cards have the same rank and suit.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-poker-hand/
// discuss: https://leetcode.com/problems/best-poker-hand/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        match suits.iter().collect::<std::collections::HashSet<_>>().len() == 1 {
            true => "Flush".to_string(),
            false => {
                let mut counter = [0; 14];
                ranks.iter().for_each(|&r| counter[r as usize] += 1);
                match counter.into_iter().max().unwrap() {
                    1 => "High Card".to_string(),
                    2 => "Pair".to_string(),
                    _ => "Three of a Kind".to_string(),
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2347_example_1() {
        let ranks = vec![13, 2, 3, 1, 9];
        let suits = vec!['a', 'a', 'a', 'a', 'a'];

        let result = "Flush".to_string();

        assert_eq!(Solution::best_hand(ranks, suits), result);
    }

    #[test]
    fn test_2347_example_2() {
        let ranks = vec![4, 4, 2, 4, 4];
        let suits = vec!['d', 'a', 'a', 'b', 'c'];

        let result = "Three of a Kind".to_string();

        assert_eq!(Solution::best_hand(ranks, suits), result);
    }

    #[test]
    fn test_2347_example_3() {
        let ranks = vec![10, 10, 2, 12, 9];
        let suits = vec!['a', 'b', 'c', 'a', 'd'];

        let result = "Pair".to_string();

        assert_eq!(Solution::best_hand(ranks, suits), result);
    }
}
