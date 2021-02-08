/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *  
 * Example 1:
 *
 * Input: s = "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()".
 *
 * Example 2:
 *
 * Input: s = ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()".
 *
 * Example 3:
 *
 * Input: s = ""
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 3 * 10^4
 * 	s[i] is '(', or ')'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut result = 0;

        for (i, ref c) in s.chars().enumerate() {
            if c == &'(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let len = i as i32 - stack[stack.len() - 1];
                    result = if result > len { result } else { len };
                }
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
    fn test_0032_example_1() {
        let s = "(()".to_string();

        assert_eq!(Solution::longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_0032_example_2() {
        let s = ")()())".to_string();

        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }

    #[test]
    fn test_0032_example_3() {
        let s = "".to_string();

        assert_eq!(Solution::longest_valid_parentheses(s), 0);
    }
}
