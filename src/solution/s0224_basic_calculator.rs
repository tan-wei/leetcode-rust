/**
 * [224] Basic Calculator
 *
 * Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.
 * Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
 *  
 * Example 1:
 *
 * Input: s = "1 + 1"
 * Output: 2
 *
 * Example 2:
 *
 * Input: s = " 2-1 + 2 "
 * Output: 3
 *
 * Example 3:
 *
 * Input: s = "(1+(4+5+2)-3)+(6+8)"
 * Output: 23
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists of digits, '+', '-', '(', ')', and ' '.
 * 	s represents a valid expression.
 * 	'+' is not used as a unary operation.
 * 	'-' could be used as a unary operation but it has to be followed by parentheses.
 * 	Every number and running calculation will fit in a signed 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator/
// discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;
        let mut sign = 1;
        let mut num = 0;
        let mut stack = vec![];
        stack.push(sign);
        for ch in s.chars() {
            match ch {
                '(' => {
                    stack.push(sign);
                }
                ')' => {
                    stack.pop();
                }
                '+' | '-' => {
                    result += sign * num;
                    sign = *stack.last().unwrap() * if ch == '+' { 1 } else { -1 };
                    num = 0;
                }
                '0'..='9' => {
                    num = num * 10 + (ch as u8 - b'0') as i32;
                }
                _ => {}
            }
        }
        result += sign * num;
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0224_example_1() {
        let s = "1 + 1".to_string();
        let result = 2;

        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_0224_example_2() {
        let s = " 2-1 + 2 ".to_string();
        let result = 3;

        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_0224_example_3() {
        let s = "(1+(4+5+2)-3)+(6+8)".to_string();
        let result = 23;

        assert_eq!(Solution::calculate(s), result);
    }
}
