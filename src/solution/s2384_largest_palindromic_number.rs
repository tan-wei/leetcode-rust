/**
 * [2384] Largest Palindromic Number
 *
 * You are given a string num consisting of digits only.
 * Return the largest palindromic integer (in the form of a string) that can be formed using digits taken from num. It should not contain leading zeroes.
 * Notes:
 *
 * 	You do not need to use all the digits of num, but you must use at least one digit.
 * 	The digits can be reordered.
 *
 *  
 * Example 1:
 *
 * Input: num = "444947137"
 * Output: "7449447"
 * Explanation:
 * Use the digits "4449477" from "<u>44494</u><u>7</u>13<u>7</u>" to form the palindromic integer "7449447".
 * It can be shown that "7449447" is the largest palindromic integer that can be formed.
 *
 * Example 2:
 *
 * Input: num = "00009"
 * Output: "9"
 * Explanation:
 * It can be shown that "9" is the largest palindromic integer that can be formed.
 * Note that the integer returned should not contain leading zeroes.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 10^5
 * 	num consists of digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-palindromic-number/
// discuss: https://leetcode.com/problems/largest-palindromic-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2384_example_1() {
        let num = "444947137".to_string();

        let result = "7449447".to_string();

        assert_eq!(Solution::largest_palindromic(num), result);
    }

    #[test]
    #[ignore]
    fn test_2384_example_2() {
        let num = "00009".to_string();

        let result = "9".to_string();

        assert_eq!(Solution::largest_palindromic(num), result);
    }
}
