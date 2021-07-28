/**
 * [233] Number of Digit One
 *
 * Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.
 *  
 * Example 1:
 *
 * Input: n = 13
 * Output: 6
 *
 * Example 2:
 *
 * Input: n = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-digit-one/
// discuss: https://leetcode.com/problems/number-of-digit-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let m: i128 = n as i128;
        let mut counter: i128 = 0;
        let mut i: i128 = 1;

        while i <= m {
            let divider: i128 = i * 10;
            counter += (m / divider) * i + std::cmp::min(std::cmp::max(m % divider - i + 1, 0), i);
            i *= 10;
        }
        return counter as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0233_example_1() {
        let n = 13;
        let result = 6;

        assert_eq!(Solution::count_digit_one(n), result);
    }

    #[test]
    fn test_0233_example_2() {
        let n = 0;
        let result = 0;

        assert_eq!(Solution::count_digit_one(n), result);
    }
}
