/**
 * [151] Reverse Words in a String
 *
 * Given an input string s, reverse the order of the words.
 * A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
 * Return a string of the words in reverse order concatenated by a single space.
 * Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.
 *  
 * Example 1:
 *
 * Input: s = "the sky is blue"
 * Output: "blue is sky the"
 *
 * Example 2:
 *
 * Input: s = "  hello world  "
 * Output: "world hello"
 * Explanation: Your reversed string should not contain leading or trailing spaces.
 *
 * Example 3:
 *
 * Input: s = "a good   example"
 * Output: "example good a"
 * Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
 *
 * Example 4:
 *
 * Input: s = "  Bob    Loves  Alice   "
 * Output: "Alice Loves Bob"
 *
 * Example 5:
 *
 * Input: s = "Alice does not even like bob"
 * Output: "bob like even not does Alice"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s contains English letters (upper-case and lower-case), digits, and spaces ' '.
 * 	There is at least one word in s.
 *
 *  
 * Follow up: Could you solve it in-place with O(1) extra space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-words-in-a-string/
// discuss: https://leetcode.com/problems/reverse-words-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0151_example_1() {
        let s = "the sky is blue".to_string();
        let result = "blue is sky the".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }

    #[test]
    fn test_0151_example_2() {
        let s = "  hello world  ".to_string();
        let result = "world hello".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }

    #[test]
    fn test_0151_example_3() {
        let s = "a good   example".to_string();
        let result = "example good a".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }

    #[test]
    fn test_0151_example_4() {
        let s = "  Bob    Loves  Alice   ".to_string();
        let result = "Alice Loves Bob".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }

    #[test]
    fn test_0151_example_5() {
        let s = "Alice does not even like bob".to_string();
        let result = "bob like even not does Alice".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }
}
