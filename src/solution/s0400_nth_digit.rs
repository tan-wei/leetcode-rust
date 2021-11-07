/**
 * [0400] Nth Digit
 *
 * Given an integer n, return the n^th digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 3
 *
 * Example 2:
 *
 * Input: n = 11
 * Output: 0
 * Explanation: The 11^th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/nth-digit/
// discuss: https://leetcode.com/problems/nth-digit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as u64;
        let mut digits = 1u64;
        let mut digit_count = 9u64;

        while n > digits * digit_count {
            n -= digits * digit_count;
            digit_count *= 10;
            digits += 1;
        }

        let index = (n - 1) % digits;
        let nth_in_digit = (n - 1) / digits;
        let num = format!("{}", 10i64.pow(digits as u32 - 1) + nth_in_digit as i64);
        num.chars().collect::<Vec<char>>()[index as usize] as i32 - '0' as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0400_example_1() {
        let n = 3;
        let result = 3;

        assert_eq!(Solution::find_nth_digit(n), result);
    }

    #[test]
    fn test_0400_example_2() {
        let n = 11;
        let result = 0;

        assert_eq!(Solution::find_nth_digit(n), result);
    }

    #[test]
    fn test_0400_additional_1() {
        let n = 1000000000;
        let result = 1;

        assert_eq!(Solution::find_nth_digit(n), result);
    }
}
