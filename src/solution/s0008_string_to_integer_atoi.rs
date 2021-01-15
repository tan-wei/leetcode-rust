/**
 * [8] String to Integer (atoi)
 *
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
 * The algorithm for myAtoi(string s) is as follows:
 * <ol>
 * 	Read in and ignore any leading whitespace.
 * 	Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
 * 	Read in next the characters until the next non-digit charcter or the end of the input is reached. The rest of the string is ignored.
 * 	Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
 * 	If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -2^31 should be clamped to -2^31, and integers greater than 2^31 - 1 should be clamped to 2^31 - 1.
 * 	Return the integer as the final result.
 * </ol>
 * Note:
 *
 * 	Only the space character ' ' is considered a whitespace character.
 * 	Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 *
 *  
 * Example 1:
 *
 * Input: str = "42"
 * Output: 42
 * Explanation: The underlined characters are what is read in, the caret is the current reader position.
 * Step 1: "42" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "42" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>42</u>" ("42" is read in)
 *            ^
 * The parsed integer is 42.
 * Since 42 is in the range [-2^31, 2^31 - 1], the final result is 42.
 *
 * Example 2:
 *
 * Input: str = "   -42"
 * Output: -42
 * Explanation:
 * Step 1: "<u>   </u>-42" (leading whitespace is read and ignored)
 *             ^
 * Step 2: "   <u>-</u>42" ('-' is read, so the result should be negative)
 *              ^
 * Step 3: "   -<u>42</u>" ("42" is read in)
 *                ^
 * The parsed integer is -42.
 * Since -42 is in the range [-2^31, 2^31 - 1], the final result is -42.
 *
 * Example 3:
 *
 * Input: str = "4193 with words"
 * Output: 4193
 * Explanation:
 * Step 1: "4193 with words" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>4193</u> with words" ("4193" is read in; reading stops because the next character is a non-digit)
 *              ^
 * The parsed integer is 4193.
 * Since 4193 is in the range [-2^31, 2^31 - 1], the final result is 4193.
 *
 * Example 4:
 *
 * Input: str = "words and 987"
 * Output: 0
 * Explanation:
 * Step 1: "words and 987" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "words and 987" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "words and 987" (reading stops immediately because there is a non-digit 'w')
 *          ^
 * The parsed integer is 0 because no digits were read.
 * Since 0 is in the range [-2^31, 2^31 - 1], the final result is 4193.
 *
 * Example 5:
 *
 * Input: str = "-91283472332"
 * Output: -2147483648
 * Explanation:
 * Step 1: "-91283472332" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "<u>-</u>91283472332" ('-' is read, so the result should be negative)
 *           ^
 * Step 3: "-<u>91283472332</u>" ("91283472332" is read in)
 *                      ^
 * The parsed integer is -91283472332.
 * Since -91283472332 is less than the lower bound of the range [-2^31, 2^31 - 1], the final result is clamped to -2^31 = -2147483648.<span style="display: none;"> </span>
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 200
 * 	s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let (n, sign) = match s.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
            Some('+') => (1, 1),
            Some(x) if x.is_digit(10) => (0, 1),
            Some('-') => (1, -1),
            _ => return 0,
        };
        let mut res = 0i32;
        let overflow = if sign > 0 {
            std::i32::MAX
        } else {
            std::i32::MIN
        };
        for c in s
            .chars()
            .skip_while(|x| x.is_whitespace())
            .skip(n)
            .take_while(|x| x.is_digit(10))
        {
            let (r, o) = res.overflowing_mul(10);
            if o {
                return overflow;
            }
            let (r, o) = r.overflowing_add(sign * (c as i32 - '0' as i32));
            if o {
                return overflow;
            }
            res = r;
        }
        res
    }

    pub fn my_atoi_v2(s: String) -> i32 {
        let mut chrs = s.chars().skip_while(|c| c == &' ').peekable();

        let sign = if chrs.peek().map_or(false, |s| s == &'-') {
            chrs.next();
            -1i32
        } else {
            if chrs.peek().map_or(false, |s| s == &'+') {
                chrs.next();
            }

            1i32
        };

        chrs.into_iter()
            .take_while(|n| n.is_numeric())
            .try_fold(0i32, |acc, n| {
                acc.checked_mul(10)
                    .and_then(|acc| acc.checked_add(n.to_digit(10).unwrap() as i32))
            })
            .map(|n| n * sign)
            .unwrap_or(if sign > 0 {
                std::i32::MAX
            } else {
                std::i32::MIN
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0008_example_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi_v2("42".to_string()), 42);
    }

    #[test]
    fn test_0008_example_2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi_v2("   -42".to_string()), -42);
    }

    #[test]
    fn test_0008_example_3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi_v2("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_0008_example_4() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi_v2("words and 987".to_string()), 0);
    }

    #[test]
    fn test_0008_example_5() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(
            Solution::my_atoi_v2("-91283472332".to_string()),
            -2147483648
        );
    }
}
