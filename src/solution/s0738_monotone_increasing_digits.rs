/**
 * [0738] Monotone Increasing Digits
 *
 * An integer has monotone increasing digits if and only if each pair of adjacent digits x and y satisfy x <= y.
 * Given an integer n, return the largest number that is less than or equal to n with monotone increasing digits.
 *  
 * Example 1:
 *
 * Input: n = 10
 * Output: 9
 *
 * Example 2:
 *
 * Input: n = 1234
 * Output: 1234
 *
 * Example 3:
 *
 * Input: n = 332
 * Output: 299
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/monotone-increasing-digits/
// discuss: https://leetcode.com/problems/monotone-increasing-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut result = 0;
        let mut prev = 9;
        let mut offset = 1;
        let mut n = n;
        while n != 0 {
            let digit = n % 10;
            if digit > prev {
                result = digit * offset - 1;
                prev = digit - 1;
            } else {
                result += digit * offset;
                prev = digit;
            }
            offset *= 10;
            n /= 10;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0738_example_1() {
        let n = 10;
        let result = 9;

        assert_eq!(Solution::monotone_increasing_digits(n), result);
    }

    #[test]
    fn test_0738_example_2() {
        let n = 1234;
        let result = 1234;

        assert_eq!(Solution::monotone_increasing_digits(n), result);
    }

    #[test]
    fn test_0738_example_3() {
        let n = 332;
        let result = 299;

        assert_eq!(Solution::monotone_increasing_digits(n), result);
    }
}
