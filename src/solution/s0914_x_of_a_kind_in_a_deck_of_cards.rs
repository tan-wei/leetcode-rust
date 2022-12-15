/**
 * [0914] X of a Kind in a Deck of Cards
 *
 * You are given an integer array deck where deck[i] represents the number written on the i^th card.
 * Partition the cards into one or more groups such that:
 *
 * 	Each group has exactly x cards where x > 1, and
 * 	All the cards in one group have the same integer written on them.
 *
 * Return true if such partition is possible, or false otherwise.
 *  
 * Example 1:
 *
 * Input: deck = [1,2,3,4,4,3,2,1]
 * Output: true
 * Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].
 *
 * Example 2:
 *
 * Input: deck = [1,1,1,2,2,2,3,3]
 * Output: false
 * Explanation: No possible partition.
 *
 *  
 * Constraints:
 *
 * 	1 <= deck.length <= 10^4
 * 	0 <= deck[i] < 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
// discuss: https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }

        let mut map = std::collections::HashMap::new();
        deck.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);
        map.values()
            .try_fold(0, |acc, &x| match gcd(x, acc) {
                1 => None,
                val => Some(val),
            })
            .is_some()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0914_example_1() {
        let deck = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let result = true;

        assert_eq!(Solution::has_groups_size_x(deck), result);
    }

    #[test]
    fn test_0914_example_2() {
        let deck = vec![1, 1, 1, 2, 2, 2, 3, 3];
        let result = false;

        assert_eq!(Solution::has_groups_size_x(deck), result);
    }
}
