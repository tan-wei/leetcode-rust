/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *  
 * Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *  
 * Constraints:
 *
 * 	1 <= num1.length, num2.length <= 200
 * 	num1 and num2 consist of digits only.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = vec![0; num1.len() + num2.len()];
        for (i, ch1) in num1.chars().rev().enumerate() {
            for (j, ch2) in num2.chars().rev().enumerate() {
                let a = ch1.to_digit(10).unwrap();
                let b = ch2.to_digit(10).unwrap();
                let lo = (a * b + result[i + j]) % 10;
                let hi = (a * b + result[i + j]) / 10;
                result[i + j] = lo;
                result[i + j + 1] += hi;
            }
        }
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }
        result
            .into_iter()
            .rev()
            .map(|s| s.to_string())
            .collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0043_example_1() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();

        assert_eq!(Solution::multiply(num1, num2), "6".to_string());
    }

    #[test]
    fn test_0043_example_2() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();

        assert_eq!(Solution::multiply(num1, num2), "56088".to_string());
    }
}
