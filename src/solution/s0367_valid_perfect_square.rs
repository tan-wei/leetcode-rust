/**
 * [367] Valid Perfect Square
 *
 * Given a positive integer num, write a function which returns True if num is a perfect square else False.
 * Follow up: Do not use any built-in library function such as sqrt.
 *  
 * Example 1:
 * Input: num = 16
 * Output: true
 * Example 2:
 * Input: num = 14
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= num <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-perfect-square/
// discuss: https://leetcode.com/problems/valid-perfect-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let mut i = 1;
        while num > 0 {
            num -= i;
            i += 2;
        }
        num == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0367_example_1() {
        let num = 16;
        let result = true;

        assert_eq!(Solution::is_perfect_square(num), result);
    }

    #[test]
    fn test_0367_example_2() {
        let num = 14;
        let result = false;

        assert_eq!(Solution::is_perfect_square(num), result);
    }
}
