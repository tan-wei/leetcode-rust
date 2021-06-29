/**
 * [204] Count Primes
 *
 * Count the number of prime numbers less than a non-negative number, n.
 *  
 * Example 1:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 *
 * Example 2:
 *
 * Input: n = 0
 * Output: 0
 *
 * Example 3:
 *
 * Input: n = 1
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 5 * 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-primes/
// discuss: https://leetcode.com/problems/count-primes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut composite = vec![false; n];
        let mut count = 0;
        for i in 2..n {
            if !composite[i] {
                count += 1;
                let mut multiple = i * i;
                while multiple < n {
                    composite[multiple] = true;
                    multiple += i;
                }
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0204_example_1() {
        let n = 10;
        let result = 4;

        assert_eq!(Solution::count_primes(n), result);
    }

    #[test]
    fn test_0204_example_2() {
        let n = 0;
        let result = 0;

        assert_eq!(Solution::count_primes(n), result);
    }

    #[test]
    fn test_0204_example_3() {
        let n = 1;
        let result = 0;

        assert_eq!(Solution::count_primes(n), result);
    }
}
