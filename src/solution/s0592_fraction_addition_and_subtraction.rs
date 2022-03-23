/**
 * [0592] Fraction Addition and Subtraction
 *
 * Given a string expression representing an expression of fraction addition and subtraction, return the calculation result in string format.
 * The final result should be an <a href="https://en.wikipedia.org/wiki/Irreducible_fraction" target="_blank">irreducible fraction</a>. If your final result is an integer, change it to the format of a fraction that has a denominator 1. So in this case, 2 should be converted to 2/1.
 *  
 * Example 1:
 *
 * Input: expression = "-1/2+1/2"
 * Output: "0/1"
 *
 * Example 2:
 *
 * Input: expression = "-1/2+1/2+1/3"
 * Output: "1/3"
 *
 * Example 3:
 *
 * Input: expression = "1/3-1/2"
 * Output: "-1/6"
 *
 *  
 * Constraints:
 *
 * 	The input string only contains '0' to '9', '/', '+' and '-'. So does the output.
 * 	Each fraction (input and output) has the format &plusmn;numerator/denominator. If the first input fraction or the output is positive, then '+' will be omitted.
 * 	The input only contains valid irreducible fractions, where the numerator and denominator of each fraction will always be in the range [1, 10]. If the denominator is 1, it means this fraction is actually an integer in a fraction format defined above.
 * 	The number of given fractions will be in the range [1, 10].
 * 	The numerator and denominator of the final result are guaranteed to be valid and in the range of 32-bit int.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-addition-and-subtraction/
// discuss: https://leetcode.com/problems/fraction-addition-and-subtraction/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://rustgym.com/leetcode/592

use std::fmt::{Display, Formatter, Result};

enum Tok {
    Op(char),
    Num(i32),
}

struct Fraction {
    sign: i32,
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(sign: i32, numerator: i32, denominator: i32) -> Self {
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
    fn add(self, other: Self) -> Self {
        let mut numerator = self.sign * self.numerator * other.denominator
            + other.sign * other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        let mut sign = 1;
        if numerator < 0 {
            sign *= -1;
            numerator *= -1;
        }
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
    fn reduce(self) -> Self {
        let sign = self.sign;
        let mut denominator = self.denominator;
        let mut numerator = self.numerator;
        let gcd = gcd(denominator, numerator);
        denominator /= gcd;
        numerator /= gcd;
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.sign > 0 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "-{}/{}", self.numerator, self.denominator)
        }
    }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}

impl Solution {
    fn fraction_addition(expression: String) -> String {
        let mut toks: Vec<Tok> = vec![];
        let mut c_it = expression.chars().peekable();
        while let Some(c) = c_it.next() {
            match c {
                '-' | '+' | '/' => {
                    toks.push(Tok::Op(c));
                }
                _ => {
                    let mut val = (c as u8 - b'0') as i32;
                    while let Some(p) = c_it.peek() {
                        if p.is_digit(10) {
                            val *= 10;
                            val += (c_it.next().unwrap() as u8 - b'0') as i32;
                        } else {
                            break;
                        }
                    }
                    toks.push(Tok::Num(val));
                }
            }
        }
        let mut t_it = toks.into_iter().peekable();
        let mut fractions: Vec<Fraction> = vec![];
        while t_it.peek().is_some() {
            let mut sign = 1;
            let numerator;
            let denominator;
            if let Some(Tok::Op('-')) = t_it.peek() {
                t_it.next().unwrap();
                sign = -1;
            }
            if let Some(Tok::Op('+')) = t_it.peek() {
                t_it.next().unwrap();
            }
            if let Tok::Num(x) = t_it.next().unwrap() {
                numerator = x;
            } else {
                panic!();
            }
            if let Tok::Op('/') = t_it.next().unwrap() {
            } else {
                panic!();
            }
            if let Tok::Num(y) = t_it.next().unwrap() {
                denominator = y;
            } else {
                panic!();
            }
            fractions.push(Fraction::new(sign, numerator, denominator));
        }
        while fractions.len() > 1 {
            let a = fractions.pop().unwrap();
            let b = fractions.pop().unwrap();
            let c = a.add(b);
            fractions.push(c);
        }
        let mut res = fractions.pop().unwrap();
        res = res.reduce();
        res.to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0592_example_1() {
        let expression = "-1/2+1/2".to_string();
        let result = "0/1".to_string();

        assert_eq!(Solution::fraction_addition(expression), result);
    }

    #[test]
    fn test_0592_example_2() {
        let expression = "-1/2+1/2+1/3".to_string();
        let result = "1/3".to_string();

        assert_eq!(Solution::fraction_addition(expression), result);
    }

    #[test]
    fn test_0592_example_3() {
        let expression = "1/3-1/2".to_string();
        let result = "-1/6".to_string();

        assert_eq!(Solution::fraction_addition(expression), result);
    }
}
