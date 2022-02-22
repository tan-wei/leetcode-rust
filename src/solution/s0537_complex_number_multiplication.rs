/**
 * [0537] Complex Number Multiplication
 *
 * A <a href="https://en.wikipedia.org/wiki/Complex_number" target="_blank">complex number</a> can be represented as a string on the form "real+imaginaryi" where:
 *
 * 	real is the real part and is an integer in the range [-100, 100].
 * 	imaginary is the imaginary part and is an integer in the range [-100, 100].
 * 	i^2 == -1.
 *
 * Given two complex numbers num1 and num2 as strings, return a string of the complex number that represents their multiplications.
 *  
 * Example 1:
 *
 * Input: num1 = "1+1i", num2 = "1+1i"
 * Output: "0+2i"
 * Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of 0+2i.
 *
 * Example 2:
 *
 * Input: num1 = "1+-1i", num2 = "1+-1i"
 * Output: "0+-2i"
 * Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.
 *
 *  
 * Constraints:
 *
 * 	num1 and num2 are valid complex numbers.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/complex-number-multiplication/
// discuss: https://leetcode.com/problems/complex-number-multiplication/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let complex_numbers = [num1, num2]
            .iter()
            .map(|s| {
                s.trim_end_matches('i')
                    .split('+')
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        format!(
            "{}+{}i",
            complex_numbers[0][0] * complex_numbers[1][0]
                - complex_numbers[0][1] * complex_numbers[1][1],
            complex_numbers[0][0] * complex_numbers[1][1]
                + complex_numbers[0][1] * complex_numbers[1][0]
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0537_example_1() {
        let num1 = "1+1i".to_string();
        let num2 = "1+1i".to_string();
        let result = "0+2i".to_string();

        assert_eq!(Solution::complex_number_multiply(num1, num2), result);
    }

    #[test]
    fn test_0537_example_2() {
        let num1 = "1+-1i".to_string();
        let num2 = "1+-1i".to_string();
        let result = "0+-2i".to_string();

        assert_eq!(Solution::complex_number_multiply(num1, num2), result);
    }
}
