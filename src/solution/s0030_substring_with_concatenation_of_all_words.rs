/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string s and an array of strings words of the same length. Return all starting indices of substring(s) in s that is a concatenation of each word in words exactly once, in any order, and without any intervening characters.
 * You can return the answer in any order.
 *  
 * Example 1:
 *
 * Input: s = "barfoothefoobarman", words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 *
 * Example 2:
 *
 * Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
 * Output: []
 *
 * Example 3:
 *
 * Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
 * Output: [6,9,12]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of lower-case English letters.
 * 	1 <= words.length <= 5000
 * 	1 <= words[i].length <= 30
 * 	words[i] consists of lower-case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// discuss: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    // Credit: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/798407/Rust-Functional-programming-solution
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        fn is_permutation(slice: &str, map: &HashMap<&str, i32>, words: &[String]) -> bool {
            let mut new_map = HashMap::new();
            let length = words[0].len();

            for i in (0..slice.len()).step_by(length) {
                let word = &slice[i..(i + length)];
                *new_map.entry(word).or_insert(0) += 1;

                if new_map.get(word) > map.get(word) {
                    return false;
                }
            }

            true
        }

        if words.len() == 0 || s.len() < words.len() * words[0].len() {
            return vec![];
        }

        let length = words.len() * words[0].len();
        let map = words
            .iter()
            .fold(HashMap::<&str, i32>::new(), |mut state, word| {
                *state.entry(word).or_insert(0) += 1;
                state
            });

        (0..=(s.len() - length))
            .filter(|index| is_permutation(&s[*index..(*index + length)], &map, &words))
            .map(|index| index as i32)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0030_example_1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec_string!["foo", "bar"];

        assert_eq_sorted!(Solution::find_substring(s, words), vec![0, 9]);
    }

    #[test]
    fn test_0030_example_2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec_string!["word", "good", "best", "word"];

        assert_eq_sorted!(Solution::find_substring(s, words), vec![]);
    }

    #[test]
    fn test_0030_example_3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec_string!["bar", "foo", "the"];

        assert_eq_sorted!(Solution::find_substring(s, words), vec![6, 9, 12]);
    }
}
