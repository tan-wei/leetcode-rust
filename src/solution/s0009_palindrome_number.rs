/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is palindrome integer.
 * An integer is a palindrome when it reads the same backward as forward. For example, 121 is palindrome while 123 is not.
 *  
 * Example 1:
 *
 * Input: x = 121
 * Output: true
 *
 * Example 2:
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 * Example 3:
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *
 * Example 4:
 *
 * Input: x = -101
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without converting the integer to a string?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }

        let x = x.to_string();
        let y = x.chars().rev().collect::<String>();

        x == y
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0009_example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_0009_example_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_0009_example_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test_0009_example_4() {
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
