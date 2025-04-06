/**
 * [1952] Three Divisors
 *
 * Given an integer n, return true if n has exactly three positive divisors. Otherwise, return false.
 * An integer m is a divisor of n if there exists an integer k such that n = k * m.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: false
 * Explantion: 2 has only two divisors: 1 and 2.
 *
 * Example 2:
 *
 * Input: n = 4
 * Output: true
 * Explantion: 4 has three divisors: 1, 2, and 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/three-divisors/
// discuss: https://leetcode.com/problems/three-divisors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_three(n: i32) -> bool {
        (1..=n).fold(0, |mut acc, x| {
            if n % x == 0 {
                acc += 1;
            }
            acc
        }) == 3
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1952_example_1() {
        let n = 2;

        let result = false;

        assert_eq!(Solution::is_three(n), result);
    }

    #[test]
    fn test_1952_example_2() {
        let n = 4;

        let result = true;

        assert_eq!(Solution::is_three(n), result);
    }
}
