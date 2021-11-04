/**
 * [0397] Integer Replacement
 *
 * Given a positive integer n, you can apply one of the following operations:
 * <ol>
 * 	If n is even, replace n with n / 2.
 * 	If n is odd, replace n with either n + 1 or n - 1.
 * </ol>
 * Return the minimum number of operations needed for n to become 1.
 *  
 * Example 1:
 *
 * Input: n = 8
 * Output: 3
 * Explanation: 8 -> 4 -> 2 -> 1
 *
 * Example 2:
 *
 * Input: n = 7
 * Output: 4
 * Explanation: 7 -> 8 -> 4 -> 2 -> 1
 * or 7 -> 6 -> 3 -> 2 -> 1
 *
 * Example 3:
 *
 * Input: n = 4
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-replacement/
// discuss: https://leetcode.com/problems/integer-replacement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut result = 0;
        let mut n = n as i64;
        while n != 1 {
            if n == 3 {
                result += 2;
                n = 1;
            } else if n % 2 == 0 {
                n /= 2;
                result += 1;
            } else if n % 4 == 1 {
                n -= 1;
                result += 1;
            } else {
                n += 1;
                result += 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0397_example_1() {
        let n = 8;
        let result = 3;

        assert_eq!(Solution::integer_replacement(n), result);
    }

    #[test]
    fn test_0397_example_2() {
        let n = 7;
        let result = 4;

        assert_eq!(Solution::integer_replacement(n), result);
    }

    #[test]
    fn test_0397_example_3() {
        let n = 4;
        let result = 2;

        assert_eq!(Solution::integer_replacement(n), result);
    }
}
