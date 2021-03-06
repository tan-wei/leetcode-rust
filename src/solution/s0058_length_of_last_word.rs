/**
 * [58] Length of Last Word
 *
 * Given a string s consists of some words separated by spaces, return the length of the last word in the string. If the last word does not exist, return 0.
 * A word is a maximal substring consisting of non-space characters only.
 *  
 * Example 1:
 * Input: s = "Hello World"
 * Output: 5
 * Example 2:
 * Input: s = " "
 * Output: 0
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of only English letters and spaces ' '.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-last-word/
// discuss: https://leetcode.com/problems/length-of-last-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace()
            .last()
            .map_or(0, |w| w.as_bytes().len()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0058_example_1() {
        let s = "Hello World".to_string();

        assert_eq!(Solution::length_of_last_word(s), 5);
    }

    #[test]
    fn test_0058_example_2() {
        let s = " ".to_string();

        assert_eq!(Solution::length_of_last_word(s), 0);
    }
}
