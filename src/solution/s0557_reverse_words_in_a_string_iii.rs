/**
 * [0557] Reverse Words in a String III
 *
 * Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
 *  
 * Example 1:
 * Input: s = "Let's take LeetCode contest"
 * Output: "s'teL ekat edoCteeL tsetnoc"
 * Example 2:
 * Input: s = "God Ding"
 * Output: "doG gniD"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	s contains printable ASCII characters.
 * 	s does not contain any leading or trailing spaces.
 * 	There is at least one word in s.
 * 	All the words in s are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-words-in-a-string-iii/
// discuss: https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ")
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0557_example_1() {
        let s = "Let's take LeetCode contest".to_string();
        let result = "s'teL ekat edoCteeL tsetnoc".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }

    #[test]
    fn test_0557_example_2() {
        let s = "God Ding".to_string();
        let result = "doG gniD".to_string();

        assert_eq!(Solution::reverse_words(s), result);
    }
}
