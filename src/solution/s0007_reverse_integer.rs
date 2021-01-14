/**
 * [7] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *  
 * Example 1:
 * Input: x = 123
 * Output: 321
 * Example 2:
 * Input: x = -123
 * Output: -321
 * Example 3:
 * Input: x = 120
 * Output: 21
 * Example 4:
 * Input: x = 0
 * Output: 0
 *  
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut cur = x;

        while cur != 0 {
            match res.checked_mul(10) {
                None => return 0,
                Some(tmp) => match tmp.checked_add(cur % 10) {
                    None => return 0,
                    Some(fine) => {
                        res = fine;
                    }
                },
            }
            cur = cur / 10;
        }

        res
    }

    pub fn reverse_v2(x: i32) -> i32 {
        x.signum()
            * x.abs()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007_example_1() {
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse_v2(-123), -321);
    }

    #[test]
    fn test_0007_example_2() {
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse_v2(120), 21);
    }

    #[test]
    fn test_0007_example_3() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse_v2(123), 321);
    }

    #[test]
    fn test_0007_example_4() {
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse_v2(0), 0);
    }
}
