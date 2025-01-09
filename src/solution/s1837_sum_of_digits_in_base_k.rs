/**
 * [1837] Sum of Digits in Base K
 *
 * Given an integer n (in base 10) and a base k, return the sum of the digits of n after converting n from base 10 to base k.
 * After converting, each digit should be interpreted as a base 10 number, and the sum should be returned in base 10.
 *  
 * Example 1:
 *
 * Input: n = 34, k = 6
 * Output: 9
 * Explanation: 34 (base 10) expressed in base 6 is 54. 5 + 4 = 9.
 *
 * Example 2:
 *
 * Input: n = 10, k = 10
 * Output: 1
 * Explanation: n is already in base 10. 1 + 0 = 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	2 <= k <= 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-digits-in-base-k/
// discuss: https://leetcode.com/problems/sum-of-digits-in-base-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        loop {
            result += n % k;
            n /= k;
            if n < k {
                break result + n;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1837_example_1() {
        let n = 34;
        let k = 6;

        let result = 9;

        assert_eq!(Solution::sum_base(n, k), result);
    }

    #[test]
    fn test_1837_example_2() {
        let n = 10;
        let k = 10;

        let result = 1;

        assert_eq!(Solution::sum_base(n, k), result);
    }
}
