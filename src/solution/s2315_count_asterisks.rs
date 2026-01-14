/**
 * [2315] Count Asterisks
 *
 * You are given a string s, where every two consecutive vertical bars '|' are grouped into a pair. In other words, the 1^st and 2^nd '|' make a pair, the 3^rd and 4^th '|' make a pair, and so forth.
 * Return the number of '*' in s, excluding the '*' between each pair of '|'.
 * Note that each '|' will belong to exactly one pair.
 *  
 * Example 1:
 *
 * Input: s = "l|*e*et|c**o|*de|"
 * Output: 2
 * Explanation: The considered characters are underlined: "<u>l</u>|*e*et|<u>c**o</u>|*de|".
 * The characters between the first and second '|' are excluded from the answer.
 * Also, the characters between the third and fourth '|' are excluded from the answer.
 * There are 2 asterisks considered. Therefore, we return 2.
 * Example 2:
 *
 * Input: s = "iamprogrammer"
 * Output: 0
 * Explanation: In this example, there are no asterisks in s. Therefore, we return 0.
 *
 * Example 3:
 *
 * Input: s = "yo|uar|e**|b|e***au|tifu|l"
 * Output: 5
 * Explanation: The considered characters are underlined: "<u>yo</u>|uar|<u>e**</u>|b|<u>e***au</u>|tifu|<u>l</u>". There are 5 asterisks considered. Therefore, we return 5.
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters, vertical bars '|', and asterisks '*'.
 * 	s contains an even number of vertical bars '|'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-asterisks/
// discuss: https://leetcode.com/problems/count-asterisks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .flat_map(|(_, s)| s.chars())
            .filter(|&c| c == '*')
            .count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2315_example_1() {
        let s = "l|*e*et|c**o|*de|".to_string();

        let result = 2;

        assert_eq!(Solution::count_asterisks(s), result);
    }

    #[test]
    fn test_2315_example_2() {
        let s = "iamprogrammer".to_string();

        let result = 0;

        assert_eq!(Solution::count_asterisks(s), result);
    }

    #[test]
    fn test_2315_example_3() {
        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();

        let result = 5;

        assert_eq!(Solution::count_asterisks(s), result);
    }
}
