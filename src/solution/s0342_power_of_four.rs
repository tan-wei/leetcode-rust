/**
 * [342] Power of Four
 *
 * Given an integer n, return true if it is a power of four. Otherwise, return false.
 * An integer n is a power of four, if there exists an integer x such that n == 4^x.
 *  
 * Example 1:
 * Input: n = 16
 * Output: true
 * Example 2:
 * Input: n = 5
 * Output: false
 * Example 3:
 * Input: n = 1
 * Output: true
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-four/
// discuss: https://leetcode.com/problems/power-of-four/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        (n & 0x55555555) != 0 && (n & (n - 1)) == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0342_example_1() {
        let n = 16;
        let result = true;

        assert_eq!(Solution::is_power_of_four(n), result);
    }

    #[test]
    fn test_0342_example_2() {
        let n = 5;
        let result = false;

        assert_eq!(Solution::is_power_of_four(n), result);
    }

    #[test]
    fn test_0342_example_3() {
        let n = 1;
        let result = true;

        assert_eq!(Solution::is_power_of_four(n), result);
    }
}
