/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *  
 * Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * Example 4:
 *
 * Input: s = ""
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut end = 0;
        let mut hash_map: HashMap<char, usize> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            if let Some(j) = hash_map.get(&c) {
                if *j >= start {
                    let cur = end - start;
                    if max < cur {
                        max = cur;
                    }

                    start = *j + 1;
                }
            }

            end += 1;
            hash_map.insert(c, i);
        }

        let cur = end - start;

        if max < cur {
            max = cur;
        }

        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0003_example_1() {
        let s = String::from("abcabcbb");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_0003_example_2() {
        let s = String::from("bbbbb");

        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test_0003_example_3() {
        let s = String::from("pwwkew");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test_0003_example_4() {
        let s = String::from("");

        assert_eq!(Solution::length_of_longest_substring(s), 0);
    }
}
