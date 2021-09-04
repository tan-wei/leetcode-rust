/**
 * [313] Super Ugly Number
 *
 * A super ugly number is a positive integer whose prime factors are in the array primes.
 * Given an integer n and an array of integers primes, return the n^th super ugly number.
 * The n^th super ugly number is guaranteed to fit in a 32-bit signed integer.
 *  
 * Example 1:
 *
 * Input: n = 12, primes = [2,7,13,19]
 * Output: 32
 * Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
 *
 * Example 2:
 *
 * Input: n = 1, primes = [2,3,5]
 * Output: 1
 * Explanation: 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^6
 * 	1 <= primes.length <= 100
 * 	2 <= primes[i] <= 1000
 * 	primes[i] is guaranteed to be a prime number.
 * 	All the values of primes are unique and sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-ugly-number/
// discuss: https://leetcode.com/problems/super-ugly-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/super-ugly-number/discuss/1349583/Rust-dp
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let m = primes.len();
        let mut pointers = vec![0; m];
        let mut dp = vec![1; n];
        for i in 1..n {
            dp[i] = (0..m)
                .map(|x| dp[pointers[x] as usize] * primes[x])
                .min()
                .unwrap();
            for x in 0..m {
                if dp[pointers[x] as usize] * primes[x] == dp[i] {
                    pointers[x] += 1;
                }
            }
        }
        dp[n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0313_exmaple_1() {
        let n = 12;
        let primes = vec![2, 7, 13, 19];
        let result = 32;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), result);
    }

    #[test]
    fn test_0313_exmaple_2() {
        let n = 1;
        let primes = vec![2, 3, 5];
        let result = 1;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), result);
    }
}
