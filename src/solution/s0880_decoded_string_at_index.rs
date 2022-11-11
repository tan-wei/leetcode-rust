/**
 * [0880] Decoded String at Index
 *
 * You are given an encoded string s. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:
 *
 * 	If the character read is a letter, that letter is written onto the tape.
 * 	If the character read is a digit d, the entire current tape is repeatedly written d - 1 more times in total.
 *
 * Given an integer k, return the k^th letter (1-indexed) in the decoded string.
 *  
 * Example 1:
 *
 * Input: s = "leet2code3", k = 10
 * Output: "o"
 * Explanation: The decoded string is "leetleetcodeleetleetcodeleetleetcode".
 * The 10^th letter in the string is "o".
 *
 * Example 2:
 *
 * Input: s = "ha22", k = 5
 * Output: "h"
 * Explanation: The decoded string is "hahahaha".
 * The 5^th letter is "h".
 *
 * Example 3:
 *
 * Input: s = "a2345678999999999999999", k = 1
 * Output: "a"
 * Explanation: The decoded string is "a" repeated 8301530446056247680 times.
 * The 1^st letter is "a".
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 100
 * 	s consists of lowercase English letters and digits 2 through 9.
 * 	s starts with a letter.
 * 	1 <= k <= 10^9
 * 	It is guaranteed that k is less than or equal to the length of the decoded string.
 * 	The decoded string is guaranteed to have less than 2^63 letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decoded-string-at-index/
// discuss: https://leetcode.com/problems/decoded-string-at-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/decoded-string-at-index/solutions/979621/rust-stack-encoding-and-reduction/
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut stack = vec![(0, String::new())];
        let mut k = k as i64;
        let mut cur_len = 0;

        for c in s.chars() {
            match c {
                c if c.is_digit(10) => {
                    let d = c.to_digit(10).unwrap();
                    let i = stack.len() - 1;
                    cur_len *= d as i64;
                    if stack[i].1.is_empty() {
                        stack[i].0 *= d as i64;
                    } else {
                        stack.push((cur_len, String::new()));
                    }
                }
                _ => {
                    let i = stack.len() - 1;
                    stack[i].1.push(c);
                    cur_len += 1;
                }
            }

            if cur_len >= k {
                break;
            }
        }

        while let Some((old_len, s2)) = stack.pop() {
            if k > old_len {
                let index = (k - old_len - 1) as usize;
                return s2.chars().nth(index).unwrap().to_string();
            } else {
                let i = stack.len() - 1;
                let p_len = stack[i].0 + (stack[i].1.len() as i64);
                k = ((k - 1) % p_len) + 1;
            }
        }
        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0880_example_1() {
        let s = "leet2code3".to_string();
        let k = 10;
        let result = "o".to_string();

        assert_eq!(Solution::decode_at_index(s, k), result);
    }

    #[test]
    fn test_0880_example_2() {
        let s = "ha22".to_string();
        let k = 5;
        let result = "h".to_string();

        assert_eq!(Solution::decode_at_index(s, k), result);
    }

    #[test]
    fn test_0880_example_3() {
        let s = "a2345678999999999999999".to_string();
        let k = 1;
        let result = "a".to_string();

        assert_eq!(Solution::decode_at_index(s, k), result);
    }
}
