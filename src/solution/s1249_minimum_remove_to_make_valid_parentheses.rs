/**
 * [1249] Minimum Remove to Make Valid Parentheses
 *
 * Given a string <font face="monospace">s</font> of '(' , ')' and lowercase English characters.
 * Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
 * Formally, a parentheses string is valid if and only if:
 *
 * 	It is the empty string, contains only lowercase characters, or
 * 	It can be written as AB (A concatenated with B), where A and B are valid strings, or
 * 	It can be written as (A), where A is a valid string.
 *
 *
 * Example 1:
 *
 * Input: s = "lee(t(c)o)de)"
 * Output: "lee(t(c)o)de"
 * Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
 *
 * Example 2:
 *
 * Input: s = "a)b(c)d"
 * Output: "ab(c)d"
 *
 * Example 3:
 *
 * Input: s = "))(("
 * Output: ""
 * Explanation: An empty string is also valid.
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either'(' , ')', or lowercase English letter.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
// discuss: https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/solutions/2285289/rust-solution/
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut temp = vec![];

        let mut l = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
                temp.push(c);
            } else if c == ')' {
                if 0 < l {
                    l -= 1;
                    temp.push(c);
                }
            } else {
                temp.push(c);
            }
        }

        let n = temp.len();
        let mut result = vec![];
        for i in (0..n).rev() {
            let c = temp[i];
            if l == 0 || c != '(' {
                result.push(c);
            } else if c == '(' {
                l -= 1;
            }
        }
        result.reverse();
        result
            .into_iter()
            .map(|v| v.to_string())
            .collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1249_example_1() {
        let s = "lee(t(c)o)de)".to_string();
        let result = "lee(t(c)o)de".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), result);
    }

    #[test]
    fn test_1249_example_2() {
        let s = "a)b(c)d".to_string();
        let result = "ab(c)d".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), result);
    }

    #[test]
    fn test_1249_example_3() {
        let s = "))((".to_string();
        let result = "".to_string();

        assert_eq!(Solution::min_remove_to_make_valid(s), result);
    }
}
