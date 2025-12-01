/**
 * [2259] Remove Digit From Number to Maximize Result
 *
 * You are given a string number representing a positive integer and a character digit.
 * Return the resulting string after removing exactly one occurrence of digit from number such that the value of the resulting string in decimal form is maximized. The test cases are generated such that digit occurs at least once in number.
 *  
 * Example 1:
 *
 * Input: number = "123", digit = "3"
 * Output: "12"
 * Explanation: There is only one '3' in "123". After removing '3', the result is "12".
 *
 * Example 2:
 *
 * Input: number = "1231", digit = "1"
 * Output: "231"
 * Explanation: We can remove the first '1' to get "231" or remove the second '1' to get "123".
 * Since 231 > 123, we return "231".
 *
 * Example 3:
 *
 * Input: number = "551", digit = "5"
 * Output: "51"
 * Explanation: We can remove either the first or second '5' from "551".
 * Both result in the string "51".
 *
 *  
 * Constraints:
 *
 * 	2 <= number.length <= 100
 * 	number consists of digits from '1' to '9'.
 * 	digit is a digit from '1' to '9'.
 * 	digit occurs at least once in number.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/
// discuss: https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut result = String::new();

        for (i, v) in number.chars().enumerate() {
            if v == digit {
                let mod_number = format!("{}{}", &number[..i], &number[i + 1..]);
                if mod_number > result {
                    result = mod_number;
                }
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
    fn test_2259_example_1() {
        let number = "123".to_string();
        let digit = '3';

        let result = "12".to_string();

        assert_eq!(Solution::remove_digit(number, digit), result);
    }

    #[test]
    fn test_2259_example_2() {
        let number = "1231".to_string();
        let digit = '1';

        let result = "231".to_string();

        assert_eq!(Solution::remove_digit(number, digit), result);
    }

    #[test]
    fn test_2259_example_3() {
        let number = "551".to_string();
        let digit = '5';

        let result = "51".to_string();

        assert_eq!(Solution::remove_digit(number, digit), result);
    }
}
