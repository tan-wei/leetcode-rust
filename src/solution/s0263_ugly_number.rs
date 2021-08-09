/**
 * [263] Ugly Number
 *
 * An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
 * Given an integer n, return true if n is an ugly number.
 *  
 * Example 1:
 *
 * Input: n = 6
 * Output: true
 * Explanation: 6 = 2 &times; 3
 * Example 2:
 *
 * Input: n = 8
 * Output: true
 * Explanation: 8 = 2 &times; 2 &times; 2
 *
 * Example 3:
 *
 * Input: n = 14
 * Output: false
 * Explanation: 14 is not ugly since it includes the prime factor 7.
 *
 * Example 4:
 *
 * Input: n = 1
 * Output: true
 * Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number/
// discuss: https://leetcode.com/problems/ugly-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        match n {
            0 => false,
            1 => true,
            n if n > 0 => {
                let mut num = n;
                while num % 2 == 0 {
                    num /= 2;
                }
                while num % 3 == 0 {
                    num /= 3;
                }
                while num % 5 == 0 {
                    num /= 5;
                }

                num == 1
            }
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0263_example_1() {
        let n = 6;
        let result = true;

        assert_eq!(Solution::is_ugly(n), result);
    }

    #[test]
    fn test_0263_example_2() {
        let n = 8;
        let result = true;

        assert_eq!(Solution::is_ugly(n), result);
    }

    #[test]
    fn test_0263_example_3() {
        let n = 14;
        let result = false;

        assert_eq!(Solution::is_ugly(n), result);
    }

    #[test]
    fn test_0263_example_4() {
        let n = 1;
        let result = true;

        assert_eq!(Solution::is_ugly(n), result);
    }
}
