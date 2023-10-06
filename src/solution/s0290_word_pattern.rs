/**
 * [290] Word Pattern
 *
 * Given a pattern and a string s, find if s follows the same pattern.
 * Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
 *  
 * Example 1:
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 *
 * Example 2:
 *
 * Input: pattern = "abba", s = "dog cat cat fish"
 * Output: false
 *
 * Example 3:
 *
 * Input: pattern = "aaaa", s = "dog cat cat dog"
 * Output: false
 *
 * Example 4:
 *
 * Input: pattern = "abba", s = "dog dog dog dog"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= pattern.length <= 300
 * 	pattern contains only lower-case English letters.
 * 	1 <= s.length <= 3000
 * 	s contains only lower-case English letters and spaces ' '.
 * 	s does not contain any leading or trailing spaces.
 * 	All the words in s are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-pattern/
// discuss: https://leetcode.com/problems/word-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut char_to_word = std::collections::HashMap::new();
        let mut word_to_char = std::collections::HashMap::new();
        let words: Vec<&str> = s.split(" ").collect();
        if pattern.len() != words.len() {
            return false;
        }
        for (i, c) in pattern.chars().enumerate() {
            if let Some(word) = char_to_word.get(&c) {
                if word != &words[i] {
                    return false;
                }
            } else {
                if let Some(v) = word_to_char.get(words[i]) {
                    if v != &c {
                        return false;
                    }
                }
                char_to_word.insert(c, words[i]);
                word_to_char.insert(words[i], c);
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0290_example_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let result = true;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn test_0290_example_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn test_0290_example_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }

    #[test]
    fn test_0290_example_4() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        let result = false;

        assert_eq!(Solution::word_pattern(pattern, s), result);
    }
}
