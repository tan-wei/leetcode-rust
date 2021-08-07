/**
 * [258] Add Digits
 *
 * Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.
 *  
 * Example 1:
 *
 * Input: num = 38
 * Output: 2
 * Explanation: The process is
 * 38 --> 3 + 8 --> 11
 * 11 --> 1 + 1 --> 2
 * Since 2 has only one digit, return it.
 *
 * Example 2:
 *
 * Input: num = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 2^31 - 1
 *
 *  
 * Follow up: Could you do it without any loop/recursion in O(1) runtime?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-digits/
// discuss: https://leetcode.com/problems/add-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let s = format!("{}", num);
        if s.len() == 1 {
            return num;
        }

        let sum = s.chars().fold(0i32, |acc, n| acc + n as i32 - '0' as i32);
        Self::add_digits(sum)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0258_example_1() {
        let num = 38;
        let result = 2;

        assert_eq!(Solution::add_digits(num), result);
    }

    #[test]
    fn test_0258_example_2() {
        let num = 0;
        let result = 0;

        assert_eq!(Solution::add_digits(num), result);
    }
}
