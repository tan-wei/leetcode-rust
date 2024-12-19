/**
 * [1808] Maximize Number of Nice Divisors
 *
 * You are given a positive integer primeFactors. You are asked to construct a positive integer n that satisfies the following conditions:
 *
 *
 *   The number of prime factors of n (not necessarily distinct) is at most primeFactors.
 *   The number of nice divisors of n is maximized. Note that a divisor of n is nice if it is divisible by every prime factor of n. For example, if n = 12, then its prime factors are [2,2,3], then 6 and 12 are nice divisors, while 3 and 4 are not.
 *
 *
 * Return the number of nice divisors of n. Since that number can be too large, return it modulo 10^9 + 7.
 *
 * Note that a prime number is a natural number greater than 1 that is not a product of two smaller natural numbers. The prime factors of a number n is a list of prime numbers such that their product equals n.
 *
 *  
 * Example 1:
 *
 *
 * Input: primeFactors = 5
 * Output: 6
 * Explanation: 200 is a valid value of n.
 * It has 5 prime factors: [2,2,2,5,5], and it has 6 nice divisors: [10,20,40,50,100,200].
 * There is not other value of n that has at most 5 prime factors and more nice divisors.
 *
 *
 * Example 2:
 *
 *
 * Input: primeFactors = 8
 * Output: 18
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= primeFactors <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-number-of-nice-divisors/
// discuss: https://leetcode.com/problems/maximize-number-of-nice-divisors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/maximize-number-of-nice-divisors/solutions/3220691/just-a-runnable-solution/
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        let prime_factors = prime_factors as i64;

        let st = [0_i64, 1, 2, 3, 4, 6];
        if prime_factors < 6 {
            return st[prime_factors as usize] as _;
        }

        let mut result = st[3 + prime_factors as usize % 3];
        let mut base = 3;
        let mut exp = prime_factors / 3 - 1;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }

        result as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1808_example_1() {
        let prime_factors = 5;

        let result = 6;

        assert_eq!(Solution::max_nice_divisors(prime_factors), result);
    }

    #[test]
    fn test_1808_example_2() {
        let prime_factors = 8;

        let result = 18;

        assert_eq!(Solution::max_nice_divisors(prime_factors), result);
    }
}
