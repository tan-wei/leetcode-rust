/**
 * [0869] Reordered Power of 2
 *
 * You are given an integer n. We reorder the digits in any order (including the original order) such that the leading digit is not zero.
 * Return true if and only if we can do this so that the resulting number is a power of two.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: true
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reordered-power-of-2/
// discuss: https://leetcode.com/problems/reordered-power-of-2/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_counts = |n: i32| -> [usize; 10] {
            let mut n = n;
            let mut d = [0; 10];
            while n > 0 {
                d[(n % 10) as usize] += 1;
                n /= 10;
            }
            d
        };
        let counts = digit_counts(n);
        (0..31).any(|i| digit_counts(1 << i) == counts)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0869_example_1() {
        let n = 1;
        let result = true;

        assert_eq!(Solution::reordered_power_of2(n), result);
    }

    #[test]
    fn test_0869_example_2() {
        let n = 10;
        let result = false;

        assert_eq!(Solution::reordered_power_of2(n), result);
    }
}
