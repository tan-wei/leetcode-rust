/**
 * [166] Fraction to Recurring Decimal
 *
 * Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
 * If the fractional part is repeating, enclose the repeating part in parentheses.
 * If multiple answers are possible, return any of them.
 * It is guaranteed that the length of the answer string is less than 10^4 for all the given inputs.
 *  
 * Example 1:
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 * Example 2:
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 * Example 3:
 * Input: numerator = 2, denominator = 3
 * Output: "0.(6)"
 * Example 4:
 * Input: numerator = 4, denominator = 333
 * Output: "0.(012)"
 * Example 5:
 * Input: numerator = 1, denominator = 5
 * Output: "0.2"
 *  
 * Constraints:
 *
 * 	-2^31 <= numerator, denominator <= 2^31 - 1
 * 	denominator != 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-to-recurring-decimal/
// discuss: https://leetcode.com/problems/fraction-to-recurring-decimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut result = if (numerator < 0) ^ (denominator < 0) {
            "-"
        } else {
            ""
        }
        .to_string();

        let (n, d) = ((numerator as i64).abs(), (denominator as i64).abs());
        let mut r = n % d;
        result += &(n / d).to_string();

        if r == 0 {
            return result;
        }

        result += &".";
        let mut indices = std::collections::HashMap::new();

        while r != 0 {
            if indices.contains_key(&r) {
                result.insert(indices[&r], '(');
                result += &")";
                return result;
            } else {
                indices.insert(r, result.len());
                r *= 10;
                result += &(r / d).to_string();
                r %= d;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0166_example_1() {
        let numerator = 1;
        let denominator = 2;
        let result = "0.5".to_string();

        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            result
        );
    }

    #[test]
    fn test_0166_example_2() {
        let numerator = 2;
        let denominator = 1;
        let result = "2".to_string();

        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            result
        );
    }

    #[test]
    fn test_0166_example_3() {
        let numerator = 2;
        let denominator = 3;
        let result = "0.(6)".to_string();

        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            result
        );
    }

    #[test]
    fn test_0166_example_4() {
        let numerator = 4;
        let denominator = 333;
        let result = "0.(012)".to_string();

        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            result
        );
    }

    #[test]
    fn test_0166_example_5() {
        let numerator = 1;
        let denominator = 5;
        let result = "0.2".to_string();

        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            result
        );
    }
}
