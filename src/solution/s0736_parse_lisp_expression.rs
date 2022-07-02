/**
 * [0736] Parse Lisp Expression
 *
 * You are given a string expression representing a Lisp-like expression to return the integer value of.
 * The syntax for these expressions is given as follows.
 *
 * 	An expression is either an integer, let expression, add expression, mult expression, or an assigned variable. Expressions always evaluate to a single integer.
 * 	(An integer could be positive or negative.)
 * 	A let expression takes the form "(let v1 e1 v2 e2 ... vn en expr)", where let is always the string "let", then there are one or more pairs of alternating variables and expressions, meaning that the first variable v1 is assigned the value of the expression e1, the second variable v2 is assigned the value of the expression e2, and so on sequentially; and then the value of this let expression is the value of the expression expr.
 * 	An add expression takes the form "(add e1 e2)" where add is always the string "add", there are always two expressions e1, e2 and the result is the addition of the evaluation of e1 and the evaluation of e2.
 * 	A mult expression takes the form "(mult e1 e2)" where mult is always the string "mult", there are always two expressions e1, e2 and the result is the multiplication of the evaluation of e1 and the evaluation of e2.
 * 	For this question, we will use a smaller subset of variable names. A variable starts with a lowercase letter, then zero or more lowercase letters or digits. Additionally, for your convenience, the names "add", "let", and "mult" are protected and will never be used as variable names.
 * 	Finally, there is the concept of scope. When an expression of a variable name is evaluated, within the context of that evaluation, the innermost scope (in terms of parentheses) is checked first for the value of that variable, and then outer scopes are checked sequentially. It is guaranteed that every expression is legal. Please see the examples for more details on the scope.
 *
 *  
 * Example 1:
 *
 * Input: expression = "(let x 2 (mult x (let x 3 y 4 (add x y))))"
 * Output: 14
 * Explanation: In the expression (add x y), when checking for the value of the variable x,
 * we check from the innermost scope to the outermost in the context of the variable we are trying to evaluate.
 * Since x = 3 is found first, the value of x is 3.
 *
 * Example 2:
 *
 * Input: expression = "(let x 3 x 2 x)"
 * Output: 2
 * Explanation: Assignment in let statements is processed sequentially.
 *
 * Example 3:
 *
 * Input: expression = "(let x 1 y 2 x (add x y) (add x y))"
 * Output: 5
 * Explanation: The first (add x y) evaluates as 3, and is assigned to x.
 * The second (add x y) evaluates as 3+2 = 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 2000
 * 	There are no leading or trailing spaces in expression.
 * 	All tokens are separated by a single space in expression.
 * 	The answer and all intermediate calculations of that answer are guaranteed to fit in a 32-bit integer.
 * 	The expression is guaranteed to be legal and evaluate to an integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/parse-lisp-expression/
// discuss: https://leetcode.com/problems/parse-lisp-expression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/parse-lisp-expression/discuss/2048231/Rust-Solution
    pub fn evaluate(expression: String) -> i32 {
        let mut expression = expression;
        expression = expression.replace("(", " ( ");
        expression = expression.replace(")", " ) ");

        Self::parse(
            &mut expression.split_whitespace().rev().collect::<Vec<&str>>(),
            std::collections::HashMap::new(),
        )
        .unwrap()
    }

    fn parse(
        expressions: &mut Vec<&str>,
        variables: std::collections::HashMap<String, i32>,
    ) -> Option<i32> {
        let mut variables = variables;
        let mut result = None;
        while let Some(expression) = expressions.pop() {
            match expression {
                ")" => {
                    return result;
                }
                "(" => {
                    return Self::parse(expressions, variables.clone());
                }
                "add" => {
                    let a = Self::parse(expressions, variables.clone()).unwrap();
                    let b = Self::parse(expressions, variables.clone()).unwrap();
                    result = Some(a + b);
                }
                "mult" => {
                    let a = Self::parse(expressions, variables.clone()).unwrap();
                    let b = Self::parse(expressions, variables.clone()).unwrap();
                    result = Some(a * b);
                }
                "let" => {
                    let mut name = None;
                    let mut let_result = None;
                    while ")" != expressions[expressions.len() - 1] {
                        if name.is_none() {
                            name = Some(expressions[expressions.len() - 1]);
                            let_result = Self::parse(expressions, variables.clone());
                            continue;
                        }
                        if name.is_some() {
                            if expressions[expressions.len() - 1] == ")" {
                                break;
                            }
                            if let Some(val) = Self::parse(expressions, variables.clone()) {
                                variables.insert(name.unwrap().to_string(), val);
                                name = None;
                                let_result = None;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    result = let_result;
                }
                c if c.parse::<i32>().is_ok() => {
                    return Some(c.parse::<i32>().unwrap());
                }
                _ => {
                    return variables.get(expression).cloned();
                }
            }
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0736_example_1() {
        let expression = "(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string();
        let result = 14;

        assert_eq!(Solution::evaluate(expression), result);
    }

    #[test]
    fn test_0736_example_2() {
        let expression = "(let x 3 x 2 x)".to_string();
        let result = 2;

        assert_eq!(Solution::evaluate(expression), result);
    }

    #[test]
    fn test_0736_example_3() {
        let expression = "(let x 1 y 2 x (add x y) (add x y))".to_string();
        let result = 5;

        assert_eq!(Solution::evaluate(expression), result);
    }
}
