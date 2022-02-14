/**
 * [0524] Longest Word in Dictionary through Deleting
 *
 * Given a string s and a string array dictionary, return the longest string in the dictionary that can be formed by deleting some of the given string characters. If there is more than one possible result, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.
 *  
 * Example 1:
 *
 * Input: s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
 * Output: "apple"
 *
 * Example 2:
 *
 * Input: s = "abpcplea", dictionary = ["a","b","c"]
 * Output: "a"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	1 <= dictionary.length <= 1000
 * 	1 <= dictionary[i].length <= 1000
 * 	s and dictionary[i] consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/
// discuss: https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dictionary = dictionary;
        dictionary.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

        let s = s.as_bytes();

        dictionary
            .into_iter()
            .find(|word| {
                let mut i = 0;
                for &b in word.as_bytes() {
                    while i < s.len() && s[i] != b {
                        i += 1;
                    }
                    if i == s.len() {
                        return false;
                    }
                    i += 1;
                }
                true
            })
            .unwrap_or_default()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0524_example_1() {
        let s = "abpcplea".to_string();
        let dictionary = vec_string!["ale", "apple", "monkey", "plea"];
        let result = "apple".to_string();

        assert_eq!(Solution::find_longest_word(s, dictionary), result);
    }

    #[test]
    fn test_0524_example_2() {
        let s = "abpcplea".to_string();
        let dictionary = vec_string!["a", "b", "c"];
        let result = "a".to_string();

        assert_eq!(Solution::find_longest_word(s, dictionary), result);
    }
}
