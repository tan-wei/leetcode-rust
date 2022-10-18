/**
 * [0856] Score of Parentheses
 *
 * Given a balanced parentheses string s, return the score of the string.
 * The score of a balanced parentheses string is based on the following rule:
 *
 * 	"()" has score 1.
 * 	AB has score A + B, where A and B are balanced parentheses strings.
 * 	(A) has score 2 * A, where A is a balanced parentheses string.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "()"
 * Output: 1
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "(())"
 * Output: 2
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "()()"
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 50
 * 	s consists of only '(' and ')'.
 * 	s is a balanced parentheses string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/score-of-parentheses/
// discuss: https://leetcode.com/problems/score-of-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0; 1];
        for c in s.chars() {
            match c {
                '(' => stack.push(0),
                ')' => {
                    if let Some(popped) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last += std::cmp::max(popped * 2, 1);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        stack[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0856_example_1() {
        let s = "()".to_string();
        let result = 1;

        assert_eq!(Solution::score_of_parentheses(s), result);
    }

    #[test]
    fn test_0856_example_2() {
        let s = "(())".to_string();
        let result = 2;

        assert_eq!(Solution::score_of_parentheses(s), result);
    }

    #[test]
    fn test_0856_example_3() {
        let s = "()()".to_string();
        let result = 2;

        assert_eq!(Solution::score_of_parentheses(s), result);
    }
}
