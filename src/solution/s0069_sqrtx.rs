/**
 * [69] Sqrt(x)
 *
 * Given a non-negative integer x, compute and return the square root of x.
 * Since the return type is an integer, the decimal digits are truncated, and only the integer part of the result is returned.
 *  
 * Example 1:
 *
 * Input: x = 4
 * Output: 2
 *
 * Example 2:
 *
 * Input: x = 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.
 *  
 * Constraints:
 *
 * 	0 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
// discuss: https://leetcode.com/problems/sqrtx/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return x;
        }

        let x = x as usize;
        let mut r = x;
        while r > x / r {
            r = (r + x / r) / 2;
        }

        r as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0069_example_1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test_0069_example_2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
