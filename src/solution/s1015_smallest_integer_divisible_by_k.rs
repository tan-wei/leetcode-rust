/**
 * [1015] Smallest Integer Divisible by K
 *
 * Given a positive integer k, you need to find the length of the smallest positive integer n such that n is divisible by k, and n only contains the digit 1.
 * Return the length of n. If there is no such n, return -1.
 * Note: n may not fit in a 64-bit signed integer.
 *  
 * Example 1:
 *
 * Input: k = 1
 * Output: 1
 * Explanation: The smallest answer is n = 1, which has length 1.
 *
 * Example 2:
 *
 * Input: k = 2
 * Output: -1
 * Explanation: There is no such positive integer n divisible by 2.
 *
 * Example 3:
 *
 * Input: k = 3
 * Output: 3
 * Explanation: The smallest answer is n = 111, which has length 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-integer-divisible-by-k/
// discuss: https://leetcode.com/problems/smallest-integer-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut m = 0;
        for i in 1..=k {
            m = (m * 10 + 1) % k;

            if m == 0 {
                return i;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1015_example_1() {
        let k = 1;
        let result = 1;

        assert_eq!(Solution::smallest_repunit_div_by_k(k), result);
    }

    #[test]
    fn test_1015_example_2() {
        let k = 2;
        let result = -1;

        assert_eq!(Solution::smallest_repunit_div_by_k(k), result);
    }

    #[test]
    fn test_1015_example_3() {
        let k = 3;
        let result = 3;

        assert_eq!(Solution::smallest_repunit_div_by_k(k), result);
    }
}
