/**
 * [2469] Convert the Temperature
 *
 * You are given a non-negative floating point number rounded to two decimal places celsius, that denotes the temperature in Celsius.
 * You should convert Celsius into Kelvin and Fahrenheit and return it as an array ans = [kelvin, fahrenheit].
 * Return the array ans. Answers within 10^-5 of the actual answer will be accepted.
 * Note that:
 *
 * 	Kelvin = Celsius + 273.15
 * 	Fahrenheit = Celsius * 1.80 + 32.00
 *
 *  
 * Example 1:
 *
 * Input: celsius = 36.50
 * Output: [309.65000,97.70000]
 * Explanation: Temperature at 36.50 Celsius converted in Kelvin is 309.65 and converted in Fahrenheit is 97.70.
 *
 * Example 2:
 *
 * Input: celsius = 122.11
 * Output: [395.26000,251.79800]
 * Explanation: Temperature at 122.11 Celsius converted in Kelvin is 395.26 and converted in Fahrenheit is 251.798.
 *
 *  
 * Constraints:
 *
 * 	0 <= celsius <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-the-temperature/
// discuss: https://leetcode.com/problems/convert-the-temperature/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2469_example_1() {
        let celsius = 36.50;

        let result = vec![309.65000, 97.70000];

        assert_f64_near!(Solution::convert_temperature(celsius)[0], result[0]);
        assert_f64_near!(Solution::convert_temperature(celsius)[1], result[1]);
    }

    #[test]
    fn test_2469_example_2() {
        let celsius = 122.11;

        let result = vec![395.26000, 251.79800];

        assert_f64_near!(Solution::convert_temperature(celsius)[0], result[0]);
        assert_f64_near!(Solution::convert_temperature(celsius)[1], result[1]);
    }
}
