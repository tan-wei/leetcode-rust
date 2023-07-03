/**
 * [1190] Reverse Substrings Between Each Pair of Parentheses
 *
 * You are given a string s that consists of lower case English letters and brackets.
 * Reverse the strings in each pair of matching parentheses, starting from the innermost one.
 * Your result should not contain any brackets.
 *  
 * Example 1:
 *
 * Input: s = "(abcd)"
 * Output: "dcba"
 *
 * Example 2:
 *
 * Input: s = "(u(love)i)"
 * Output: "iloveu"
 * Explanation: The substring "love" is reversed first, then the whole string is reversed.
 *
 * Example 3:
 *
 * Input: s = "(ed(et(oc))el)"
 * Output: "leetcode"
 * Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s only contains lower case English characters and parentheses.
 * 	It is guaranteed that all parentheses are balanced.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// discuss: https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![vec![]];
        for c in s.chars() {
            if c == '(' {
                stack.push(vec![]);
            } else if c == ')' {
                let mut arr = stack.pop().unwrap();
                arr.reverse();
                if stack.is_empty() {
                    stack.push(arr);
                } else {
                    let li = stack.len() - 1;
                    for cc in arr {
                        stack[li].push(cc);
                    }
                }
            } else {
                let li = stack.len() - 1;
                stack[li].push(c);
            }
        }

        let mut result = vec![];
        for arr in stack {
            for c in arr {
                result.push(c);
            }
        }

        result.iter().map(|v| v.to_string()).collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1190_example_1() {
        let s = "(abcd)".to_string();
        let result = "dcba".to_string();

        assert_eq!(Solution::reverse_parentheses(s), result);
    }

    #[test]
    fn test_1190_example_2() {
        let s = "(u(love)i)".to_string();
        let result = "iloveu".to_string();

        assert_eq!(Solution::reverse_parentheses(s), result);
    }

    #[test]
    fn test_1190_example_3() {
        let s = "(ed(et(oc))el)".to_string();
        let result = "leetcode".to_string();

        assert_eq!(Solution::reverse_parentheses(s), result);
    }
}
