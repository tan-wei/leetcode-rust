/**
 * [227] Basic Calculator II
 *
 * Given a string s which represents an expression, evaluate this expression and return its value.
 * The integer division should truncate toward zero.
 * <b data-stringify-type="bold">Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as <code data-stringify-type="code">eval().
 *  
 * Example 1:
 * Input: s = "3+2*2"
 * Output: 7
 * Example 2:
 * Input: s = " 3/2 "
 * Output: 1
 * Example 3:
 * Input: s = " 3+5 / 2 "
 * Output: 5
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
 * 	s represents a valid expression.
 * 	All the integers in the expression are non-negative integers in the range [0, 2^31 - 1].
 * 	The answer is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator-ii/
// discuss: https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut s = s;
        s.push('.');
        s.chars()
            .fold((vec![], 0i32, '+'), |(mut s, mut n, mut op), c| {
                if c == ' ' {
                    return (s, n, op);
                }
                match c.to_digit(10) {
                    Some(digit) => n = n * 10 + digit as i32,
                    _ => {
                        match op {
                            '+' => s.push(n),
                            '-' => s.push(n * -1),
                            '/' => *s.last_mut().unwrap() /= n,
                            '*' => *s.last_mut().unwrap() *= n,
                            _ => {}
                        }
                        op = c;
                        n = 0;
                    }
                }
                (s, n, op)
            })
            .0
            .iter()
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0227_example_1() {
        let s = "3+2*2".to_string();
        let result = 7;

        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_0227_example_2() {
        let s = " 3/2 ".to_string();
        let result = 1;

        assert_eq!(Solution::calculate(s), result);
    }

    #[test]
    fn test_0227_example_3() {
        let s = " 3+5 / 2 ".to_string();
        let result = 5;

        assert_eq!(Solution::calculate(s), result);
    }
}
