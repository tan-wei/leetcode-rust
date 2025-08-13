/**
 * [2119] A Number After a Double Reversal
 *
 * Reversing an integer means to reverse all its digits.
 *
 * 	For example, reversing 2021 gives 1202. Reversing 12300 gives 321 as the leading zeros are not retained.
 *
 * Given an integer num, reverse num to get reversed1, then reverse reversed1 to get reversed2. Return true if reversed2 equals num. Otherwise return false.
 *  
 * Example 1:
 *
 * Input: num = 526
 * Output: true
 * Explanation: Reverse num to get 625, then reverse 625 to get 526, which equals num.
 *
 * Example 2:
 *
 * Input: num = 1800
 * Output: false
 * Explanation: Reverse num to get 81, then reverse 81 to get 18, which does not equal num.
 *
 * Example 3:
 *
 * Input: num = 0
 * Output: true
 * Explanation: Reverse num to get 0, then reverse 0 to get 0, which equals num.
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/a-number-after-a-double-reversal/
// discuss: https://leetcode.com/problems/a-number-after-a-double-reversal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2119_example_1() {
        let num = 526;

        let result = true;

        assert_eq!(Solution::is_same_after_reversals(num), result);
    }

    #[test]
    fn test_2119_example_2() {
        let num = 1800;

        let result = false;

        assert_eq!(Solution::is_same_after_reversals(num), result);
    }

    #[test]
    fn test_2119_example_3() {
        let num = 0;

        let result = true;

        assert_eq!(Solution::is_same_after_reversals(num), result);
    }
}
