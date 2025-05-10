/**
 * [1996] The Number of Weak Characters in the Game
 *
 * You are playing a game that contains multiple characters, and each of the characters has two main properties: attack and defense. You are given a 2D integer array properties where properties[i] = [attacki, defensei] represents the properties of the i^th character in the game.
 * A character is said to be weak if any other character has both attack and defense levels strictly greater than this character's attack and defense levels. More formally, a character i is said to be weak if there exists another character j where attackj > attacki and defensej > defensei.
 * Return the number of weak characters.
 *  
 * Example 1:
 *
 * Input: properties = [[5,5],[6,3],[3,6]]
 * Output: 0
 * Explanation: No character has strictly greater attack and defense than the other.
 *
 * Example 2:
 *
 * Input: properties = [[2,2],[3,3]]
 * Output: 1
 * Explanation: The first character is weak because the second character has a strictly greater attack and defense.
 *
 * Example 3:
 *
 * Input: properties = [[1,5],[10,4],[4,3]]
 * Output: 1
 * Explanation: The third character is weak because the second character has a strictly greater attack and defense.
 *
 *  
 * Constraints:
 *
 * 	2 <= properties.length <= 10^5
 * 	properties[i].length == 2
 * 	1 <= attacki, defensei <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/
// discuss: https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by_key(|p| (-p[0], p[1]));
        properties
            .iter()
            .fold((0, -1), |(count, max_def), prop| {
                if max_def > prop[1] {
                    (count + 1, max_def)
                } else {
                    (count, prop[1])
                }
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1996_example_1() {
        let properties = vec![vec![5, 5], vec![6, 3], vec![3, 6]];

        let result = 0;

        assert_eq!(Solution::number_of_weak_characters(properties), result);
    }

    #[test]
    fn test_1996_example_2() {
        let properties = vec![vec![2, 2], vec![3, 3]];

        let result = 1;

        assert_eq!(Solution::number_of_weak_characters(properties), result);
    }

    #[test]
    fn test_1996_example_3() {
        let properties = vec![vec![1, 5], vec![10, 4], vec![4, 3]];

        let result = 1;

        assert_eq!(Solution::number_of_weak_characters(properties), result);
    }
}
