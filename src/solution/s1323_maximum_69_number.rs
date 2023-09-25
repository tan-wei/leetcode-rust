/**
 * [1323] Maximum 69 Number
 *
 * You are given a positive integer num consisting only of digits 6 and 9.
 * Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
 *
 * Example 1:
 *
 * Input: num = 9669
 * Output: 9969
 * Explanation:
 * Changing the first digit results in 6669.
 * Changing the second digit results in 9969.
 * Changing the third digit results in 9699.
 * Changing the fourth digit results in 9666.
 * The maximum number is 9969.
 *
 * Example 2:
 *
 * Input: num = 9996
 * Output: 9999
 * Explanation: Changing the last digit 6 to 9 results in the maximum number.
 *
 * Example 3:
 *
 * Input: num = 9999
 * Output: 9999
 * Explanation: It is better not to apply any change.
 *
 *
 * Constraints:
 *
 * 	1 <= num <= 10^4
 * 	num consists of only 6 and 9 digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-69-number/
// discuss: https://leetcode.com/problems/maximum-69-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        match num {
            _ if num / 1000 == 6 => num + 3000,
            _ if (num % 1000) / 100 == 6 => num + 300,
            _ if (num % 100) / 10 == 6 => num + 30,
            _ if (num % 10) == 6 => num + 3,
            _ => num,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1323_example_1() {
        let num = 9669;
        let result = 9969;

        assert_eq!(Solution::maximum69_number(num), result);
    }

    #[test]
    fn test_1323_example_2() {
        let num = 9999;
        let result = 9999;

        assert_eq!(Solution::maximum69_number(num), result);
    }
}
