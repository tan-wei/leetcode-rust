/**
 * [65] Valid Number
 *
 * A valid number can be split up into these components (in order):
 * <ol>
 * 	A decimal number or an integer.
 * 	(Optional) An 'e' or 'E', followed by an integer.
 * </ol>
 * A decimal number can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	One of the following formats:
 * 	<ol>
 * 		At least one digit, followed by a dot '.'.
 * 		At least one digit, followed by a dot '.', followed by at least one digit.
 * 		A dot '.', followed by at least one digit.
 * 	</ol>
 *
 * </ol>
 * An integer can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	At least one digit.
 * </ol>
 * For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].
 * Given a string s, return true if s is a valid number.
 *  
 * Example 1:
 *
 * Input: s = "0"
 * Output: true
 *
 * Example 2:
 *
 * Input: s = "e"
 * Output: false
 *
 * Example 3:
 *
 * Input: s = "."
 * Output: false
 *
 * Example 4:
 *
 * Input: s = ".1"
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 20
 * 	s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-number/
// discuss: https://leetcode.com/problems/valid-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_number(s: String) -> bool {
        enum State {
            Initial,
            BaseInt(bool),
            BaseAfterPoint(bool),
            Exp(bool),
            ExpDigits(bool),
            Trailing(bool),
            Failure,
        }

        use State::*;

        let mut state = Initial;

        for c in s.chars() {
            state = match state {
                Initial => match c {
                    ' ' => Initial,
                    '-' | '+' => BaseInt(false),
                    '.' => BaseAfterPoint(false),
                    '0'..='9' => BaseInt(true),
                    _ => Failure,
                },
                BaseInt(f) => match c {
                    '0'..='9' => BaseInt(true),
                    '.' => BaseAfterPoint(f),
                    'e' | 'E' => Exp(f),
                    ' ' => Trailing(f),
                    _ => Failure,
                },
                BaseAfterPoint(f) => match c {
                    '0'..='9' => BaseAfterPoint(true),
                    'e' | 'E' => Exp(f),
                    ' ' => Trailing(f),
                    _ => Failure,
                },
                Exp(f) => match c {
                    '0'..='9' if f => ExpDigits(true),
                    '-' | '+' if f => ExpDigits(false),
                    _ => Failure,
                },
                ExpDigits(f) => match c {
                    '0'..='9' => ExpDigits(true),
                    ' ' => Trailing(f),
                    _ => Failure,
                },
                Trailing(f) => match c {
                    ' ' => Trailing(f),
                    _ => Failure,
                },
                Failure => Failure,
            };
            match state {
                Failure => break,
                _ => continue,
            };
        }

        match state {
            Trailing(f) | BaseInt(f) | BaseAfterPoint(f) | ExpDigits(f) => f,
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0065_example_1() {
        let s = "0".to_string();
        let result = true;

        assert_eq!(Solution::is_number(s), result);
    }

    #[test]
    fn test_0065_example_2() {
        let s = "e".to_string();
        let result = false;

        assert_eq!(Solution::is_number(s), result);
    }

    #[test]
    fn test_0065_example_3() {
        let s = ".".to_string();
        let result = false;

        assert_eq!(Solution::is_number(s), result);
    }

    #[test]
    fn test_0065_example_4() {
        let s = ".1".to_string();
        let result = true;

        assert_eq!(Solution::is_number(s), result);
    }

    #[test]
    fn test_0065_case_inf() {
        let s = "inf".to_string();
        let result = false;

        assert_eq!(Solution::is_number(s), result);
    }

    #[test]
    fn test_0065_case_big_number() {
        let s = "-8115e957".to_string();
        let result = true;

        assert_eq!(Solution::is_number(s), result);
    }
}
