/**
 * [1021] Remove Outermost Parentheses
 *
 * A valid parentheses string is either empty "", "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.
 *
 * 	For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.
 *
 * A valid parentheses string s is primitive if it is nonempty, and there does not exist a way to split it into s = A + B, with A and B nonempty valid parentheses strings.
 * Given a valid parentheses string s, consider its primitive decomposition: s = P1 + P2 + ... + Pk, where Pi are primitive valid parentheses strings.
 * Return s after removing the outermost parentheses of every primitive string in the primitive decomposition of s.
 *  
 * Example 1:
 *
 * Input: s = "(()())(())"
 * Output: "()()()"
 * Explanation:
 * The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
 * After removing outer parentheses of each part, this is "()()" + "()" = "()()()".
 *
 * Example 2:
 *
 * Input: s = "(()())(())(()(()))"
 * Output: "()()()()(())"
 * Explanation:
 * The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
 * After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".
 *
 * Example 3:
 *
 * Input: s = "()()"
 * Output: ""
 * Explanation:
 * The input string is "()()", with primitive decomposition "()" + "()".
 * After removing outer parentheses of each part, this is "" + "" = "".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '(' or ')'.
 * 	s is a valid parentheses string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-outermost-parentheses/
// discuss: https://leetcode.com/problems/remove-outermost-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut opened = 0;

        for c in s.chars() {
            if c == '(' {
                if opened > 0 {
                    result.push(c);
                }
                opened += 1;
            }
            if c == ')' {
                if opened > 1 {
                    result.push(c);
                }
                opened -= 1;
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
    fn test_1021_example_1() {
        let s = "(()())(())".to_string();
        let result = "()()()".to_string();

        assert_eq!(Solution::remove_outer_parentheses(s), result);
    }

    #[test]
    fn test_1021_example_2() {
        let s = "(()())(())(()(()))".to_string();
        let result = "()()()()(())".to_string();

        assert_eq!(Solution::remove_outer_parentheses(s), result);
    }

    #[test]
    fn test_1021_example_3() {
        let s = "()()".to_string();
        let result = "".to_string();

        assert_eq!(Solution::remove_outer_parentheses(s), result);
    }
}
