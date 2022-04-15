/**
 * [0640] Solve the Equation
 *
 * Solve a given equation and return the value of 'x' in the form of a string "x=#value". The equation contains only '+', '-' operation, the variable 'x' and its coefficient. You should return "No solution" if there is no solution for the equation, or "Infinite solutions" if there are infinite solutions for the equation.
 * If there is exactly one solution for the equation, we ensure that the value of 'x' is an integer.
 *  
 * Example 1:
 *
 * Input: equation = "x+5-3+x=6+x-2"
 * Output: "x=2"
 *
 * Example 2:
 *
 * Input: equation = "x=x"
 * Output: "Infinite solutions"
 *
 * Example 3:
 *
 * Input: equation = "2x=x"
 * Output: "x=0"
 *
 *  
 * Constraints:
 *
 * 	3 <= equation.length <= 1000
 * 	equation has exactly one '='.
 * 	equation consists of integers with an absolute value in the range [0, 100] without any leading zeros, and the variable 'x'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/solve-the-equation/
// discuss: https://leetcode.com/problems/solve-the-equation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/solve-the-equation/discuss/553226/Rust-Clean-solution.-0ms-time-and-100-space

#[derive(Debug)]
struct Expression {
    coefficient: i32,
    term: i32,
}

impl Expression {
    fn new(s: &str) -> Self {
        let s = s
            .replace("-", "+-")
            .replace("-x", "-1x")
            .replace("+x", "+1x");

        let parts: Vec<&str> = s.split("+").filter(|p| p.len() > 0).collect();

        let coefficient = parts
            .iter()
            .filter(|p| p.ends_with("x"))
            .map(|p| &p[..p.len() - 1])
            .map(|p| p.parse::<i32>().unwrap_or(1))
            .sum();

        let term = parts
            .iter()
            .filter(|p| !p.ends_with("x"))
            .map(|p| p.parse::<i32>().unwrap())
            .sum();

        Expression { coefficient, term }
    }

    fn solve(&self) -> String {
        if self.coefficient == 0 && self.term == 0 {
            String::from("Infinite solutions")
        } else if self.coefficient == 0 {
            String::from("No solution")
        } else {
            let result = -self.term / self.coefficient;
            format!("x={}", result)
        }
    }
}

impl std::ops::Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Expression {
        Expression {
            coefficient: self.coefficient - rhs.coefficient,
            term: self.term - rhs.term,
        }
    }
}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let parts: Vec<&str> = equation.split("=").collect();
        assert!(parts.len() == 2);

        let left = Expression::new(parts[0]);
        let right = Expression::new(parts[1]);
        let result = left - right;

        result.solve()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0640_example_1() {
        let equation = "x+5-3+x=6+x-2".to_string();
        let result = "x=2".to_string();

        assert_eq!(Solution::solve_equation(equation), result);
    }

    #[test]
    fn test_0640_example_2() {
        let equation = "x=x".to_string();
        let result = "Infinite solutions".to_string();

        assert_eq!(Solution::solve_equation(equation), result);
    }

    #[test]
    fn test_0640_example_3() {
        let equation = "2x=x".to_string();
        let result = "x=0".to_string();

        assert_eq!(Solution::solve_equation(equation), result);
    }
}
