/**
 * [1961] Check If String Is a Prefix of Array
 *
 * Given a string s and an array of strings words, determine whether s is a prefix string of words.
 * A string s is a prefix string of words if s can be made by concatenating the first k strings in words for some positive k no larger than words.length.
 * Return true if s is a prefix string of words, or false otherwise.
 *  
 * Example 1:
 *
 * Input: s = "iloveleetcode", words = ["i","love","leetcode","apples"]
 * Output: true
 * Explanation:
 * s can be made by concatenating "i", "love", and "leetcode" together.
 *
 * Example 2:
 *
 * Input: s = "iloveleetcode", words = ["apples","i","love","leetcode"]
 * Output: false
 * Explanation:
 * It is impossible to make s using a prefix of arr.
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 20
 * 	1 <= s.length <= 1000
 * 	words[i] and s consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/
// discuss: https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let close = words.iter().fold(String::new(), |acc, item| {
            if acc.len() < s.len() {
                format!("{}{}", acc, item)
            } else {
                acc
            }
        });

        close == s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_1961_example_1() {
        let s = "iloveleetcode".to_string();
        let words = vec_string!["i", "love", "leetcode", "apples"];

        let result = true;

        assert_eq!(Solution::is_prefix_string(s, words), result);
    }

    #[test]
    fn test_1961_example_2() {
        let s = "iloveleetcode".to_string();
        let words = vec_string!["apples", "i", "love", "leetcode"];

        let result = false;

        assert_eq!(Solution::is_prefix_string(s, words), result);
    }
}
