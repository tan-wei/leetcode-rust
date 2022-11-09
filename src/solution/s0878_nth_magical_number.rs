/**
 * [0878] Nth Magical Number
 *
 * A positive integer is magical if it is divisible by either a or b.
 * Given the three integers n, a, and b, return the n^th magical number. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1, a = 2, b = 3
 * Output: 2
 *
 * Example 2:
 *
 * Input: n = 4, a = 2, b = 3
 * Output: 6
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 * 	2 <= a, b <= 4 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/nth-magical-number/
// discuss: https://leetcode.com/problems/nth-magical-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/nth-magical-number/solutions/1623878/rust-binary-search-solution-100-for-space-and-time/
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let a: i64 = a as i64;
        let b: i64 = b as i64;
        let n: i64 = n as i64;

        let mut lo = 2;
        let mut hi = (n * std::cmp::min(a, b));

        let lcm = a * b / Solution::gcd(a, b);

        while lo < hi {
            let mi = lo + (hi - lo) / 2;

            let val = (mi / a) + (mi / b) - (mi / lcm);

            if val < n {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }

        (lo % MOD) as i32
    }

    fn gcd(a: i64, b: i64) -> i64 {
        let r = a % b;
        if r == 0 {
            return b;
        }
        Solution::gcd(b, r)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0878_example_1() {
        let n = 1;
        let a = 2;
        let b = 3;
        let result = 2;

        assert_eq!(Solution::nth_magical_number(n, a, b), result);
    }

    #[test]
    fn test_0878_example_2() {
        let n = 4;
        let a = 2;
        let b = 3;
        let result = 6;

        assert_eq!(Solution::nth_magical_number(n, a, b), result);
    }
}
