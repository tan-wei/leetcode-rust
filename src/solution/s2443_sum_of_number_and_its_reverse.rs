/**
 * [2443] Sum of Number and Its Reverse
 *
 * Given a non-negative integer num, return true if num can be expressed as the sum of any non-negative integer and its reverse, or false otherwise.
 *  
 * Example 1:
 *
 * Input: num = 443
 * Output: true
 * Explanation: 172 + 271 = 443 so we return true.
 *
 * Example 2:
 *
 * Input: num = 63
 * Output: false
 * Explanation: 63 cannot be expressed as the sum of a non-negative integer and its reverse so we return false.
 *
 * Example 3:
 *
 * Input: num = 181
 * Output: true
 * Explanation: 140 + 041 = 181 so we return true. Note that when a number is reversed, there may be leading zeros.
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-number-and-its-reverse/
// discuss: https://leetcode.com/problems/sum-of-number-and-its-reverse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        Self::dfs_helper(num, num, 0)
    }

    fn dfs_helper(num: i32, rem: i32, curr: i32) -> bool {
        if rem == 0 {
            let mut n = curr;
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            return rev + curr == num;
        }

        for i in 0..=9 {
            if Self::dfs_helper(num, rem / 10, curr * 10 + i) {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2443_example_1() {
        let num = 443;

        let result = true;

        assert_eq!(Solution::sum_of_number_and_reverse(num), result);
    }

    #[test]
    fn test_2443_example_2() {
        let num = 63;

        let result = false;

        assert_eq!(Solution::sum_of_number_and_reverse(num), result);
    }

    #[test]
    fn test_2443_example_3() {
        let num = 181;

        let result = true;

        assert_eq!(Solution::sum_of_number_and_reverse(num), result);
    }
}
