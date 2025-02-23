/**
 * [1896] Minimum Cost to Change the Final Value of Expression
 *
 * You are given a valid boolean expression as a string expression consisting of the characters '1','0','&amp;' (bitwise AND operator),'|' (bitwise OR operator),'(', and ')'.
 *
 * 	For example, "()1|1" and "(1)&amp;()" are not valid while "1", "(((1))|(0))", and "1|(0&amp;(1))" are valid expressions.
 *
 * Return the minimum cost to change the final value of the expression.
 *
 * 	For example, if expression = "1|1|(0&amp;0)&amp;1", its value is 1|1|(0&amp;0)&amp;1 = 1|1|0&amp;1 = 1|0&amp;1 = 1&amp;1 = 1. We want to apply operations so that the new expression evaluates to 0.
 *
 * The cost of changing the final value of an expression is the number of operations performed on the expression. The types of operations are described as follows:
 *
 * 	Turn a '1' into a '0'.
 * 	Turn a '0' into a '1'.
 * 	Turn a '&amp;' into a '|'.
 * 	Turn a '|' into a '&amp;'.
 *
 * Note: '&amp;' does not take precedence over '|' in the order of calculation. Evaluate parentheses first, then in left-to-right order.
 *  
 * Example 1:
 *
 * Input: expression = "1&amp;(0|1)"
 * Output: 1
 * Explanation: We can turn "1&amp;(0<u>|</u>1)" into "1&amp;(0<u>&amp;</u>1)" by changing the '|' to a '&amp;' using 1 operation.
 * The new expression evaluates to 0.
 *
 * Example 2:
 *
 * Input: expression = "(0&amp;0)&amp;(0&amp;0&amp;0)"
 * Output: 3
 * Explanation: We can turn "(0<u>&amp;0</u>)<u>&amp;</u>(0&amp;0&amp;0)" into "(0<u>|1</u>)<u>|</u>(0&amp;0&amp;0)" using 3 operations.
 * The new expression evaluates to 1.
 *
 * Example 3:
 *
 * Input: expression = "(0|(1|0&amp;1))"
 * Output: 1
 * Explanation: We can turn "(0|(<u>1</u>|0&amp;1))" into "(0|(<u>0</u>|0&amp;1))" using 1 operation.
 * The new expression evaluates to 0.
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 10^5
 * 	expression only contains '1','0','&amp;','|','(', and ')'
 * 	All parentheses are properly matched.
 * 	There will be no empty parentheses (i.e: "()" is not a substring of expression).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-change-the-final-value-of-expression/
// discuss: https://leetcode.com/problems/minimum-cost-to-change-the-final-value-of-expression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1896_example_1() {
        let expression = "1&(0|1)".to_string();

        let result = 1;

        assert_eq!(Solution::min_operations_to_flip(expression), result);
    }

    #[test]
    #[ignore]
    fn test_1896_example_2() {
        let expression = "(0&0)&(0&0&0)".to_string();

        let result = 3;

        assert_eq!(Solution::min_operations_to_flip(expression), result);
    }

    #[test]
    #[ignore]
    fn test_1896_example_3() {
        let expression = "(0|(1|0&1))".to_string();

        let result = 1;

        assert_eq!(Solution::min_operations_to_flip(expression), result);
    }
}
