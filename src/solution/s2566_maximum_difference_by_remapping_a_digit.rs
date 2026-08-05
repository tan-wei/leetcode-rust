/**
 * [2566] Maximum Difference by Remapping a Digit
 *
 * You are given an integer num. You know that Bob will sneakily remap one of the 10 possible digits (0 to 9) to another digit.
 * Return the difference between the maximum and minimum values Bob can make by remapping exactly one digit in num.
 * Notes:
 *
 * 	When Bob remaps a digit <font face="monospace">d1</font> to another digit <font face="monospace">d2</font>, Bob replaces all occurrences of d1 in num with d2.
 * 	Bob can remap a digit to itself, in which case num does not change.
 * 	Bob can remap different digits for obtaining minimum and maximum values respectively.
 * 	The resulting number after remapping can contain leading zeroes.
 *
 *  
 * Example 1:
 *
 * Input: num = 11891
 * Output: 99009
 * Explanation:
 * To achieve the maximum value, Bob can remap the digit 1 to the digit 9 to yield 99899.
 * To achieve the minimum value, Bob can remap the digit 1 to the digit 0, yielding 890.
 * The difference between these two numbers is 99009.
 *
 * Example 2:
 *
 * Input: num = 90
 * Output: 99
 * Explanation:
 * The maximum value that can be returned by the function is 99 (if 0 is replaced by 9) and the minimum value that can be returned by the function is 0 (if 9 is replaced by 0).
 * Thus, we return 99.
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/
// discuss: https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let nums = num.to_string();
        nums.chars()
            .find(|&c| c != '9')
            .map_or(num, |c| nums.replace(c, "9").parse().unwrap())
            - nums
                .replace(nums.get(0..1).unwrap(), "0")
                .parse::<i32>()
                .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2566_example_1() {
        let num = 11891;

        let result = 99009;

        assert_eq!(Solution::min_max_difference(num), result);
    }

    #[test]
    fn test_2566_example_2() {
        let num = 90;

        let result = 99;

        assert_eq!(Solution::min_max_difference(num), result);
    }
}
