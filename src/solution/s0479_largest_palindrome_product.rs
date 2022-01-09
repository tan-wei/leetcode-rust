/**
 * [0479] Largest Palindrome Product
 *
 * Given an integer n, return the largest palindromic integer that can be represented as the product of two n-digits integers. Since the answer can be very large, return it modulo 1337.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: 987
 * Explanation: 99 x 91 = 9009, 9009 % 1337 = 987
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-palindrome-product/
// discuss: https://leetcode.com/problems/largest-palindrome-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        match n {
            1 => 9,
            n => {
                let lower = 10i32.pow(n as u32 - 1);
                let upper = 10i32.pow(n as u32) - 1;

                for i in (lower..=upper).rev() {
                    let cand = Self::build_palindrome(i);

                    let mut j: usize = upper as usize;

                    while j * j >= cand {
                        if cand % j == 0 && cand / j <= upper as usize {
                            return (cand % 1337usize) as i32;
                        }
                        j -= 1;
                    }
                }

                return -1;
            }
        }
    }

    fn build_palindrome(n: i32) -> usize {
        let s = format!("{}", n);
        let r = s.chars().rev().collect::<String>();
        (s + &r).parse().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0479_example_1() {
        let n = 2;
        let result = 987;

        assert_eq!(Solution::largest_palindrome(n), result);
    }

    #[test]
    fn test_0479_example_2() {
        let n = 1;
        let result = 9;

        assert_eq!(Solution::largest_palindrome(n), result);
    }

    #[test]
    fn test_0479_additional_1() {
        let n = 8;
        let result = 475;

        assert_eq!(Solution::largest_palindrome(n), result);
    }

    #[test]
    fn test_0479_additional_2() {
        let n = 7;
        let result = 877;

        assert_eq!(Solution::largest_palindrome(n), result);
    }
}
