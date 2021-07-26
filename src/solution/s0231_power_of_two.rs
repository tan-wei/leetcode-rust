/**
 * [231] Power of Two
 *
 * Given an integer n, return true if it is a power of two. Otherwise, return false.
 * An integer n is a power of two, if there exists an integer x such that n == 2^x.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: true
 * Explanation: 2^0 = 1
 *
 * Example 2:
 *
 * Input: n = 16
 * Output: true
 * Explanation: 2^4 = 16
 *
 * Example 3:
 *
 * Input: n = 3
 * Output: false
 *
 * Example 4:
 *
 * Input: n = 4
 * Output: true
 *
 * Example 5:
 *
 * Input: n = 5
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-two/
// discuss: https://leetcode.com/problems/power-of-two/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0231_example_1() {
        let n = 1;
        let result = true;

        assert_eq!(Solution::is_power_of_two(n), result);
    }

    #[test]
    fn test_0231_example_2() {
        let n = 16;
        let result = true;

        assert_eq!(Solution::is_power_of_two(n), result);
    }

    #[test]
    fn test_0231_example_3() {
        let n = 3;
        let result = false;

        assert_eq!(Solution::is_power_of_two(n), result);
    }

    #[test]
    fn test_0231_example_4() {
        let n = 4;
        let result = true;

        assert_eq!(Solution::is_power_of_two(n), result);
    }

    #[test]
    fn test_0231_example_5() {
        let n = 5;
        let result = false;

        assert_eq!(Solution::is_power_of_two(n), result);
    }
}
