/**
 * [1844] Replace All Digits with Characters
 *
 * You are given a 0-indexed string s that has lowercase English letters in its even indices and digits in its odd indices.
 * You must perform an operation shift(c, x), where c is a character and x is a digit, that returns the x^th character after c.
 *
 * 	For example, shift('a', 5) = 'f' and shift('x', 0) = 'x'.
 *
 * For every odd index i, you want to replace the digit s[i] with the result of the shift(s[i-1], s[i]) operation.
 * Return s after replacing all digits. It is guaranteed that shift(s[i-1], s[i]) will never exceed 'z'.
 * Note that shift(c, x) is not a preloaded function, but an operation to be implemented as part of the solution.
 *  
 * Example 1:
 *
 * Input: s = "a1c1e1"
 * Output: "abcdef"
 * Explanation: The digits are replaced as follows:
 * - s[1] -> shift('a',1) = 'b'
 * - s[3] -> shift('c',1) = 'd'
 * - s[5] -> shift('e',1) = 'f'
 * Example 2:
 *
 * Input: s = "a1b2c3d4e"
 * Output: "abbdcfdhe"
 * Explanation: The digits are replaced as follows:
 * - s[1] -> shift('a',1) = 'b'
 * - s[3] -> shift('b',2) = 'd'
 * - s[5] -> shift('c',3) = 'f'
 * - s[7] -> shift('d',4) = 'h'
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists only of lowercase English letters and digits.
 * 	shift(s[i-1], s[i]) <= 'z' for all odd indices i.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-all-digits-with-characters/
// discuss: https://leetcode.com/problems/replace-all-digits-with-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn replace_digits(s: String) -> String {
        s.bytes()
            .fold(
                (Vec::new(), None),
                |(mut chars, preshift), byte| match preshift {
                    Some(prev) => {
                        let output = byte + (prev - b'0');
                        chars.push(output as char);
                        (chars, None)
                    }
                    None => {
                        chars.push(byte as char);
                        (chars, Some(byte))
                    }
                },
            )
            .0
            .iter()
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1844_example_1() {
        let s = "a1c1e1".to_string();

        let result = "abcdef".to_string();

        assert_eq!(Solution::replace_digits(s), result);
    }

    #[test]
    fn test_1844_example_2() {
        let s = "a1b2c3d4e".to_string();

        let result = "abbdcfdhe".to_string();

        assert_eq!(Solution::replace_digits(s), result);
    }
}
