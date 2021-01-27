/**
 * [20] Valid Parentheses
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 * An input string is valid if:
 * <ol>
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 * </ol>
 *  
 * Example 1:
 *
 * Input: s = "()"
 * Output: true
 *
 * Example 2:
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 * Example 3:
 *
 * Input: s = "(]"
 * Output: false
 *
 * Example 4:
 *
 * Input: s = "([)]"
 * Output: false
 *
 * Example 5:
 *
 * Input: s = "{[]}"
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of parentheses only '()[]{}'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if Some(c) != stack.pop() => return false,
                _ => continue, // otherwise, Some(c) == stcack.pop()
            }
        }

        stack.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0020_example_1() {
        let s = "()".to_string();
        let result = true;

        assert_eq!(Solution::is_valid(s), result);
    }

    #[test]
    fn test_0020_example_2() {
        let s = "()[]{}".to_string();
        let result = true;

        assert_eq!(Solution::is_valid(s), result);
    }

    #[test]
    fn test_0020_example_3() {
        let s = "(]".to_string();
        let result = false;

        assert_eq!(Solution::is_valid(s), result);
    }

    #[test]
    fn test_0020_example_4() {
        let s = "([)]".to_string();
        let result = false;

        assert_eq!(Solution::is_valid(s), result);
    }

    #[test]
    fn test_0020_example_5() {
        let s = "{[]}".to_string();
        let result = true;

        assert_eq!(Solution::is_valid(s), result);
    }
}
