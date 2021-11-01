/**
 * [0394] Decode String
 *
 * Given an encoded string, return its decoded string.
 * The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
 * You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
 * Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
 *  
 * Example 1:
 * Input: s = "3[a]2[bc]"
 * Output: "aaabcbc"
 * Example 2:
 * Input: s = "3[a2[c]]"
 * Output: "accaccacc"
 * Example 3:
 * Input: s = "2[abc]3[cd]ef"
 * Output: "abcabccdcdcdef"
 * Example 4:
 * Input: s = "abc3[cd]xyz"
 * Output: "abccdcdcdxyz"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 30
 * 	s consists of lowercase English letters, digits, and square brackets '[]'.
 * 	s is guaranteed to be a valid input.
 * 	All the integers in s are in the range [1, 300].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-string/
// discuss: https://leetcode.com/problems/decode-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let (mut n, mut result) = (0, String::new());

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push((n, result.clone()));
                    n = 0;
                    result.clear();
                }
                ']' => {
                    if let Some(last) = stack.pop() {
                        result = last.1 + result.repeat(last.0).as_str();
                    }
                }
                '0'..='9' => n = n * 10 + (c as u8 - b'0') as usize,
                c => result.push(c),
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0394_example_1() {
        let s = "3[a]2[bc]".to_string();
        let result = "aaabcbc".to_string();

        assert_eq!(Solution::decode_string(s), result);
    }

    #[test]
    fn test_0394_example_2() {
        let s = "3[a2[c]]".to_string();
        let result = "accaccacc".to_string();

        assert_eq!(Solution::decode_string(s), result);
    }

    #[test]
    fn test_0394_example_3() {
        let s = "2[abc]3[cd]ef".to_string();
        let result = "abcabccdcdcdef".to_string();

        assert_eq!(Solution::decode_string(s), result);
    }

    #[test]
    fn test_0394_example_4() {
        let s = "abc3[cd]xyz".to_string();
        let result = "abccdcdcdxyz".to_string();

        assert_eq!(Solution::decode_string(s), result);
    }
}
