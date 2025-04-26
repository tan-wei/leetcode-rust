/**
 * [1977] Number of Ways to Separate Numbers
 *
 * You wrote down many positive integers in a string called num. However, you realized that you forgot to add commas to seperate the different numbers. You remember that the list of integers was non-decreasing and that no integer had leading zeros.
 * Return the number of possible lists of integers that you could have written down to get the string num. Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: num = "327"
 * Output: 2
 * Explanation: You could have written down the numbers:
 * 3, 27
 * 327
 *
 * Example 2:
 *
 * Input: num = "094"
 * Output: 0
 * Explanation: No numbers can have leading zeros and all numbers must be positive.
 *
 * Example 3:
 *
 * Input: num = "0"
 * Output: 0
 * Explanation: No numbers can have leading zeros and all numbers must be positive.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 3500
 * 	num consists of digits '0' through '9'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-separate-numbers/
// discuss: https://leetcode.com/problems/number-of-ways-to-separate-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1977_example_1() {
        let num = "327".to_string();

        let result = 2;

        assert_eq!(Solution::number_of_combinations(num), result);
    }

    #[test]
    #[ignore]
    fn test_1977_example_2() {
        let num = "094".to_string();

        let result = 0;

        assert_eq!(Solution::number_of_combinations(num), result);
    }

    #[test]
    #[ignore]
    fn test_1977_example_3() {
        let num = "0".to_string();

        let result = 0;

        assert_eq!(Solution::number_of_combinations(num), result);
    }
}
