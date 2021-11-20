/**
 * [0415] Add Strings
 *
 * Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
 * You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
 *  
 * Example 1:
 *
 * Input: num1 = "11", num2 = "123"
 * Output: "134"
 *
 * Example 2:
 *
 * Input: num1 = "456", num2 = "77"
 * Output: "533"
 *
 * Example 3:
 *
 * Input: num1 = "0", num2 = "0"
 * Output: "0"
 *
 *  
 * Constraints:
 *
 * 	1 <= num1.length, num2.length <= 10^4
 * 	num1 and num2 consist of only digits.
 * 	num1 and num2 don't have any leading zeros except for the zero itself.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-strings/
// discuss: https://leetcode.com/problems/add-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num1 = num1.bytes().rev();
        let mut num2 = num2.bytes().rev();
        let mut v = Vec::with_capacity(num1.len().max(num2.len()) + 1);
        let mut carry = false;
        loop {
            let n1 = num1.next().map(|u| u - b'0');
            let n2 = num2.next().map(|u| u - b'0');
            if n1.is_none() && n2.is_none() && !carry {
                break;
            }
            let d = if carry { 1 } else { 0 } + n1.unwrap_or_default() + n2.unwrap_or_default();
            carry = d > 9;
            v.push((b'0' + d % 10) as char);
        }
        v.iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0415_example_1() {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        let result = "134".to_string();

        assert_eq!(Solution::add_strings(num1, num2), result);
    }

    #[test]
    fn test_0415_example_2() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        let result = "533".to_string();

        assert_eq!(Solution::add_strings(num1, num2), result);
    }

    #[test]
    fn test_0415_example_3() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        let result = "0".to_string();

        assert_eq!(Solution::add_strings(num1, num2), result);
    }
}
