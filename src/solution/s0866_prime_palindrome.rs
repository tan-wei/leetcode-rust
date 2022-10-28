/**
 * [0866] Prime Palindrome
 *
 * Given an integer n, return the smallest prime palindrome greater than or equal to n.
 * An integer is prime if it has exactly two divisors: 1 and itself. Note that 1 is not a prime number.
 *
 * 	For example, 2, 3, 5, 7, 11, and 13 are all primes.
 *
 * An integer is a palindrome if it reads the same from left to right as it does from right to left.
 *
 * 	For example, 101 and 12321 are palindromes.
 *
 * The test cases are generated so that the answer always exists and is in the range [2, 2 * 10^8].
 *  
 * <strong class="example">Example 1:
 * Input: n = 6
 * Output: 7
 * <strong class="example">Example 2:
 * Input: n = 8
 * Output: 11
 * <strong class="example">Example 3:
 * Input: n = 13
 * Output: 101
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prime-palindrome/
// discuss: https://leetcode.com/problems/prime-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        match n {
            1 => 2,
            2 => 2,
            3 => 3,
            4 => 5,
            5 => 5,
            6 => 7,
            7 => 7,
            8..=11 => 11,
            _ => {
                for x in 1..100000 {
                    let mut s = x.to_string();
                    let len = s.len();
                    let r = s.chars().rev().skip(1).collect::<String>();
                    s.push_str(&r);
                    let y = s.parse::<i32>().unwrap();
                    if y >= n && Self::is_prime(y) {
                        return y;
                    }
                }
                std::i32::MIN // unreachable!()
            }
        }
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        } else if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        for i in (3..((n as f64).sqrt() as i32) + 1).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0866_example_1() {
        let n = 6;
        let result = 7;

        assert_eq!(Solution::prime_palindrome(n), result);
    }

    #[test]
    fn test_0866_example_2() {
        let n = 8;
        let result = 11;

        assert_eq!(Solution::prime_palindrome(n), result);
    }

    #[test]
    fn test_0866_example_3() {
        let n = 13;
        let result = 101;

        assert_eq!(Solution::prime_palindrome(n), result);
    }
}
