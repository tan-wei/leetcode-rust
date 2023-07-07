/**
 * [1201] Ugly Number III
 *
 * An ugly number is a positive integer that is divisible by a, b, or c.
 * Given four integers n, a, b, and c, return the n^th ugly number.
 *  
 * Example 1:
 *
 * Input: n = 3, a = 2, b = 3, c = 5
 * Output: 4
 * Explanation: The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3^rd is 4.
 *
 * Example 2:
 *
 * Input: n = 4, a = 2, b = 3, c = 4
 * Output: 6
 * Explanation: The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4^th is 6.
 *
 * Example 3:
 *
 * Input: n = 5, a = 2, b = 11, c = 13
 * Output: 10
 * Explanation: The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5^th is 10.
 *
 *  
 * Constraints:
 *
 * 	1 <= n, a, b, c <= 10^9
 * 	1 <= a * b * c <= 10^18
 * 	It is guaranteed that the result will be in range [1, 2 * 10^9].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number-iii/
// discuss: https://leetcode.com/problems/ugly-number-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let (a, b, c) = (a as i64, b as i64, c as i64);
        let lcm_ab = Solution::lcm(a, b);
        let lcm_bc = Solution::lcm(b, c);
        let lcm_ac = Solution::lcm(a, c);
        let lcm_abc = Solution::lcm(lcm_ab, c);

        let mut lo: i64 = 1;
        let mut hi: i64 = 2 * 1e9 as i64;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = mid / a + mid / b + mid / c - mid / lcm_ab - mid / lcm_bc - mid / lcm_ac
                + mid / lcm_abc;
            if cnt < n as i64 {
                lo = mid + 1
            } else {
                hi = mid
            }
        }
        lo as i32
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        a * b / Solution::gcd(a, b)
    }

    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        x = x.abs();
        y = y.abs();
        while y != 0 {
            // let (x, y) = (y, x % y);  This will not work! due to rust's binding and ownership.
            let tmp = x;
            x = y;
            y = tmp % y;
        }
        x
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1201_example_1() {
        let n = 3;
        let a = 2;
        let b = 3;
        let c = 5;
        let result = 4;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), result);
    }

    #[test]
    fn test_1201_example_2() {
        let n = 4;
        let a = 2;
        let b = 3;
        let c = 4;
        let result = 6;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), result);
    }

    #[test]
    fn test_1201_example_3() {
        let n = 5;
        let a = 2;
        let b = 11;
        let c = 13;
        let result = 10;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), result);
    }
}
