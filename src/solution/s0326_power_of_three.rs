/**
 * [326] Power of Three
 *
 * Given an integer n, return true if it is a power of three. Otherwise, return false.
 * An integer n is a power of three, if there exists an integer x such that n == 3^x.
 *  
 * Example 1:
 * Input: n = 27
 * Output: true
 * Example 2:
 * Input: n = 0
 * Output: false
 * Example 3:
 * Input: n = 9
 * Output: true
 * Example 4:
 * Input: n = 45
 * Output: false
 *  
 * Constraints:
 *
 * 	-2^31 <= n <= 2^31 - 1
 *
 *  
 * Follow up: Could you solve it without loops/recursion?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-three/
// discuss: https://leetcode.com/problems/power-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        if n > 1 {
            while n % 3 == 0 {
                n /= 3;
            }
        }
        n == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0326_example_1() {
        let n = 27;
        let result = true;

        assert_eq!(Solution::is_power_of_three(n), result);
    }

    #[test]
    fn test_0326_example_2() {
        let n = 0;
        let result = false;

        assert_eq!(Solution::is_power_of_three(n), result);
    }

    #[test]
    fn test_0326_example_3() {
        let n = 9;
        let result = true;

        assert_eq!(Solution::is_power_of_three(n), result);
    }

    #[test]
    fn test_0326_example_4() {
        let n = 45;
        let result = false;

        assert_eq!(Solution::is_power_of_three(n), result);
    }
}
