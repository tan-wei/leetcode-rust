/**
 * [0770] Basic Calculator IV
 *
 * Given an expression such as expression = "e + 8 - a + 5" and an evaluation map such as {"e": 1} (given in terms of evalvars = ["e"] and evalints = [1]), return a list of tokens representing the simplified expression, such as ["-1*a","14"]
 *
 * 	An expression alternates chunks and symbols, with a space separating each chunk and symbol.
 * 	A chunk is either an expression in parentheses, a variable, or a non-negative integer.
 * 	A variable is a string of lowercase letters (not including digits.) Note that variables can be multiple letters, and note that variables never have a leading coefficient or unary operator like "2x" or "-x".
 *
 * Expressions are evaluated in the usual order: brackets first, then multiplication, then addition and subtraction.
 *
 * 	For example, expression = "1 + 2 * 3" has an answer of ["7"].
 *
 * The format of the output is as follows:
 *
 * 	For each term of free variables with a non-zero coefficient, we write the free variables within a term in sorted order lexicographically.
 *
 * 		For example, we would never write a term like "b*a*c", only "a*b*c".
 *
 *
 * 	Terms have degrees equal to the number of free variables being multiplied, counting multiplicity. We write the largest degree terms of our answer first, breaking ties by lexicographic order ignoring the leading coefficient of the term.
 *
 * 		For example, "a*a*b*c" has degree 4.
 *
 *
 * 	The leading coefficient of the term is placed directly to the left with an asterisk separating it from the variables (if they exist.) A leading coefficient of 1 is still printed.
 * 	An example of a well-formatted answer is ["-2*a*a*a", "3*a*a*b", "3*b*b", "4*a", "5*c", "-6"].
 * 	Terms (including constant terms) with coefficient 0 are not included.
 *
 * 		For example, an expression of "0" has an output of [].
 *
 *
 *
 * Note: You may assume that the given expression is always valid. All intermediate results will be in the range of [-2^31, 2^31 - 1].
 *  
 * Example 1:
 *
 * Input: expression = "e + 8 - a + 5", evalvars = ["e"], evalints = [1]
 * Output: ["-1*a","14"]
 *
 * Example 2:
 *
 * Input: expression = "e - 8 + temperature - pressure", evalvars = ["e", "temperature"], evalints = [1, 12]
 * Output: ["-1*pressure","5"]
 *
 * Example 3:
 *
 * Input: expression = "(e + 8) * (e - 8)", evalvars = [], evalints = []
 * Output: ["1*e*e","-64"]
 *
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 250
 * 	expression consists of lowercase English letters, digits, '+', '-', '*', '(', ')', ' '.
 * 	expression does not contain any leading or trailing spaces.
 * 	All the tokens in expression are separated by a single space.
 * 	0 <= evalvars.length <= 100
 * 	1 <= evalvars[i].length <= 20
 * 	evalvars[i] consists of lowercase English letters.
 * 	evalints.length == evalvars.length
 * 	-100 <= evalints[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator-iv/
// discuss: https://leetcode.com/problems/basic-calculator-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/basic-calculator-iv/discuss/1141959/Rust-100-100

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monomial {
    members: std::collections::HashMap<String, i32>,
}

impl Monomial {
    fn new() -> Self {
        Self {
            members: std::collections::HashMap::new(),
        }
    }

    fn set_member(&mut self, name: String, coefficient: i32) {
        if coefficient == 0 {
            return;
        }

        let ret = self.members.insert(name, coefficient);
        assert!(ret.is_none());
    }

    fn multiply(m1: &Monomial, m2: &Monomial) -> Self {
        let mut result = m1.clone();

        for (variable, coefficient) in m2.members.iter() {
            let cnt = result.members.entry(variable.clone()).or_insert(0);
            *cnt += coefficient;
        }

        result
    }

    fn output(&self) -> (i32, Vec<String>, String) {
        let mut degree = 0;
        let mut vars = vec![];
        let mut result = String::new();

        let mut members: Vec<_> = self.members.iter().collect();
        members.sort();

        for (var, coeff) in members {
            for _ in 0..*coeff {
                vars.push(var.clone());

                result.push('*');
                result.push_str(var);
            }
            degree += coeff;
        }

        (degree, vars, result)
    }
}

impl std::hash::Hash for Monomial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.members.iter().collect();
        pairs.sort();

        for (k, v) in pairs {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
struct Polynomial {
    members: std::collections::HashMap<Monomial, i32>,
}

impl Polynomial {
    fn new() -> Self {
        Self {
            members: std::collections::HashMap::new(),
        }
    }

    fn set_member(&mut self, member: Monomial, coefficient: i32) {
        if coefficient == 0 {
            return;
        }
        let ret = self.members.insert(member, coefficient);
        assert!(ret.is_none());
    }

    fn add(p1: &Polynomial, p2: &Polynomial, subtract: bool) -> Polynomial {
        let mut result = p1.clone();

        let mut op = if subtract { -1 } else { 1 };

        for (mono, coefficient) in p2.members.iter() {
            let cnt = result.members.entry(mono.clone()).or_insert(0);
            *cnt += op * coefficient;

            if *cnt == 0 {
                result.members.remove(mono);
            }
        }

        result
    }

    fn mult(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
        let mut result = Self::new();

        let p1: Vec<_> = p1.members.iter().collect();
        let p2: Vec<_> = p2.members.iter().collect();

        for (m1, coeff1) in p1.iter() {
            for (m2, coeff2) in p2.iter() {
                let m = Monomial::multiply(m1, m2);
                let coeff = *coeff1 * *coeff2;

                let mut p = Self::new();
                p.set_member(m, coeff);

                result = Self::add(&result, &p, false);
            }
        }

        result
    }

    fn output(&self) -> Vec<String> {
        let mut parts = vec![];

        for (mono, coeff) in self.members.iter() {
            let (degree, vars, mut out) = mono.output();
            out = coeff.to_string() + &out;

            parts.push((std::cmp::Reverse(degree), vars, out));
        }

        parts.sort();

        parts.into_iter().map(|(_, _, out)| out).collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Var(String),
    Op(char),
    Group(Vec<Token>),
}

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        let mut constants = std::collections::HashMap::new();

        for (name, value) in evalvars.into_iter().zip(evalints.into_iter()) {
            constants.insert(name, value);
        }

        let mut iter = expression.chars().peekable();
        let mut it = &mut iter;

        let tokens = Self::parse_tokens(it);

        let eval_result = Self::eval(tokens.as_slice(), &constants);

        eval_result.output()
    }

    fn parse_tokens<I: Iterator<Item = char>>(it: &mut core::iter::Peekable<I>) -> Vec<Token> {
        // variable operator variable operator variable

        let mut tokens = vec![];

        let member = Self::parse_member(it).unwrap();
        tokens.push(member);

        while let Some(op) = Self::parse_operator(it) {
            tokens.push(Token::Op(op));
            let member = Self::parse_member(it).unwrap();
            tokens.push(member);
        }

        tokens
    }

    fn parse_member<I: Iterator<Item = char>>(it: &mut core::iter::Peekable<I>) -> Option<Token> {
        Self::parse_spaces(it);

        if it.peek().copied() == Some('(') {
            it.next();
            let tokens = Self::parse_tokens(it);
            Some(Token::Group(tokens))
        } else {
            let mut variable = String::new();
            while let Some(c) = it.peek().copied() {
                if c.is_alphanumeric() {
                    variable.push(c);
                    it.next();
                } else {
                    break;
                }
            }

            if variable.is_empty() {
                None
            } else {
                Some(Token::Var(variable))
            }
        }
    }

    fn parse_operator<I: Iterator<Item = char>>(it: &mut core::iter::Peekable<I>) -> Option<char> {
        Self::parse_spaces(it);

        if it.peek().copied() == Some(')') {
            it.next();
            None
        } else {
            if let Some(c) = it.peek().copied() {
                if c == '+' || c == '-' || c == '*' {
                    it.next();
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn parse_spaces<I: Iterator<Item = char>>(it: &mut core::iter::Peekable<I>) {
        while it.peek().copied() == Some(' ') {
            it.next();
        }
    }

    fn eval(tokens: &[Token], constants: &std::collections::HashMap<String, i32>) -> Polynomial {
        if tokens.len() == 1 {
            Self::eval_single(&tokens[0], constants)
        } else {
            let mut members = vec![];
            let mut operators = vec![];

            for i in 0..tokens.len() {
                if i % 2 == 0 {
                    members.push(Self::eval_single(&tokens[i], constants));
                } else {
                    if let Token::Op(c) = tokens[i] {
                        operators.push(c);
                    } else {
                        panic!()
                    }
                }
            }

            let mut new_members = vec![];
            let mut new_operators = vec![];

            for op in ['*', '-', '+'].iter() {
                {
                    let mut it_mem = members.drain(..);
                    let mut it_op = operators.drain(..);

                    new_members.push(it_mem.next().unwrap());

                    while let (Some(mem), Some(c)) = (it_mem.next(), it_op.next()) {
                        if c == *op {
                            let a = new_members.pop().unwrap();
                            let b = mem;

                            let res = match op {
                                '*' => Polynomial::mult(&a, &b),
                                '+' => Polynomial::add(&a, &b, false),
                                '-' => Polynomial::add(&a, &b, true),
                                _ => panic!(),
                            };

                            new_members.push(res);
                        } else {
                            new_members.push(mem);
                            new_operators.push(c);
                        }
                    }
                }

                std::mem::swap(&mut members, &mut new_members);
                std::mem::swap(&mut operators, &mut new_operators);
            }

            assert!(operators.is_empty());
            assert!(members.len() == 1);

            members.into_iter().next().unwrap()
        }
    }

    fn eval_single(
        token: &Token,
        constants: &std::collections::HashMap<String, i32>,
    ) -> Polynomial {
        match token {
            Token::Var(variable) => {
                let constant = constants.get(variable).copied();
                let num: Option<i32> = variable.parse().ok();

                let mut poly = Polynomial::new();

                if let Some(num) = num.or(constant) {
                    let mono = Monomial::new();
                    poly.set_member(mono, num);
                } else {
                    let mut mono = Monomial::new();
                    mono.set_member(variable.clone(), 1);
                    poly.set_member(mono, 1);
                }

                poly
            }
            Token::Group(tokens) => Self::eval(tokens.as_slice(), constants),
            _ => {
                panic!()
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0770_example_1() {
        let expression = "e + 8 - a + 5".to_string();
        let evalvars = vec_string!["e"];
        let evalints = vec![1];
        let result = vec_string!["-1*a", "14"];

        assert_eq!(
            Solution::basic_calculator_iv(expression, evalvars, evalints),
            result
        );
    }

    #[test]
    fn test_0770_example_2() {
        let expression = "e - 8 + temperature - pressure".to_string();
        let evalvars = vec_string!["e", "temperature"];
        let evalints = vec![1, 12];
        let result = vec_string!["-1*pressure", "5"];

        assert_eq!(
            Solution::basic_calculator_iv(expression, evalvars, evalints),
            result
        );
    }

    #[test]
    fn test_0770_example_3() {
        let expression = "(e + 8) * (e - 8)".to_string();
        let evalvars = vec_string![];
        let evalints = vec![];
        let result = vec_string!["1*e*e", "-64"];

        assert_eq!(
            Solution::basic_calculator_iv(expression, evalvars, evalints),
            result
        );
    }
}
