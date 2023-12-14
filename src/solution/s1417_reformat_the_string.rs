/**
 * [1417] Reformat The String
 *
 * You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).
 * You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.
 * Return the reformatted string or return an empty string if it is impossible to reformat the string.
 *  
 * Example 1:
 *
 * Input: s = "a0b1c2"
 * Output: "0a1b2c"
 * Explanation: No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.
 *
 * Example 2:
 *
 * Input: s = "leetcode"
 * Output: ""
 * Explanation: "leetcode" has only characters so we cannot separate them by digits.
 *
 * Example 3:
 *
 * Input: s = "1229857369"
 * Output: ""
 * Explanation: "1229857369" has only digits so we cannot separate them by characters.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of only lowercase English letters and/or digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reformat-the-string/
// discuss: https://leetcode.com/problems/reformat-the-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reformat(s: String) -> String {
        let num_digits = s.chars().filter(|c| c.is_ascii_digit()).count() as i32;
        let num_chars = s.len() as i32 - num_digits;

        if (num_digits - num_chars).abs() > 1 {
            return "".to_owned();
        }

        let mut result = vec![' '; s.len()];
        let (mut ind_letter, mut ind_digit) = match num_chars >= num_digits {
            true => (0, 1),
            false => (1, 0),
        };

        for c in s.chars() {
            match c.is_ascii_digit() {
                true => {
                    result[ind_digit] = c;
                    ind_digit += 2
                }
                false => {
                    result[ind_letter] = c;
                    ind_letter += 2
                }
            }
        }
        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1417_example_1() {
        let s = "a0b1c2".to_string();

        let result = "0a1b2c".to_string();

        assert_eq!(Solution::reformat(s), result);
    }

    #[test]
    #[ignore]
    fn test_1417_example_2() {
        let s = "leetcode".to_string();

        let result = "".to_string();

        assert_eq!(Solution::reformat(s), result);
    }

    #[test]
    #[ignore]
    fn test_1417_example_3() {
        let s = "1229857369".to_string();

        let result = "".to_string();

        assert_eq!(Solution::reformat(s), result);
    }
}
