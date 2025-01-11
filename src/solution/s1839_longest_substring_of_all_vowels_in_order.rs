/**
 * [1839] Longest Substring Of All Vowels in Order
 *
 * A string is considered beautiful if it satisfies the following conditions:
 *
 * 	Each of the 5 English vowels ('a', 'e', 'i', 'o', 'u') must appear at least once in it.
 * 	The letters must be sorted in alphabetical order (i.e. all 'a's before 'e's, all 'e's before 'i's, etc.).
 *
 * For example, strings "aeiou" and "aaaaaaeiiiioou" are considered beautiful, but "uaeio", "aeoiu", and "aaaeeeooo" are not beautiful.
 * Given a string word consisting of English vowels, return the length of the longest beautiful substring of word. If no such substring exists, return 0.
 * A substring is a contiguous sequence of characters in a string.
 *  
 * Example 1:
 *
 * Input: word = "aeiaaio<u>aaaaeiiiiouuu</u>ooaauuaeiu"
 * Output: 13
 * Explanation: The longest beautiful substring in word is "aaaaeiiiiouuu" of length 13.
 * Example 2:
 *
 * Input: word = "aeeeiiiioooauuu<u>aeiou</u>"
 * Output: 5
 * Explanation: The longest beautiful substring in word is "aeiou" of length 5.
 *
 * Example 3:
 *
 * Input: word = "a"
 * Output: 0
 * Explanation: There is no beautiful substring, so return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 5 * 10^5
 * 	word consists of characters 'a', 'e', 'i', 'o', and 'u'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-of-all-vowels-in-order/
// discuss: https://leetcode.com/problems/longest-substring-of-all-vowels-in-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1839_example_1() {
        let word = "aeiaaioaaaaeiiiiouuooaauuaeiu".to_string();

        let result = 13;

        assert_eq!(Solution::longest_beautiful_substring(word), result);
    }

    #[test]
    #[ignore]
    fn test_1839_example_2() {
        let word = "aeeeiiiioooauuuaeiou".to_string();

        let result = 5;

        assert_eq!(Solution::longest_beautiful_substring(word), result);
    }

    #[test]
    #[ignore]
    fn test_1839_example_3() {
        let word = "a".to_string();

        let result = 0;

        assert_eq!(Solution::longest_beautiful_substring(word), result);
    }
}
