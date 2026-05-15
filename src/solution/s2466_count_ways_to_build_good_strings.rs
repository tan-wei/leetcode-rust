/**
 * [2466] Count Ways To Build Good Strings
 *
 * Given the integers zero, one, low, and high, we can construct a string by starting with an empty string, and then at each step perform either of the following:
 *
 * 	Append the character '0' zero times.
 * 	Append the character '1' one times.
 *
 * This can be performed any number of times.
 * A good string is a string constructed by the above process having a length between low and high (inclusive).
 * Return the number of different good strings that can be constructed satisfying these properties. Since the answer can be large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: low = 3, high = 3, zero = 1, one = 1
 * Output: 8
 * Explanation:
 * One possible valid good string is "011".
 * It can be constructed as follows: "" -> "0" -> "01" -> "011".
 * All binary strings from "000" to "111" are good strings in this example.
 *
 * Example 2:
 *
 * Input: low = 2, high = 3, zero = 1, one = 2
 * Output: 5
 * Explanation: The good strings are "00", "11", "000", "110", and "011".
 *
 *  
 * Constraints:
 *
 * 	1 <= low <= high <= 10^5
 * 	1 <= zero, one <= low
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-ways-to-build-good-strings/
// discuss: https://leetcode.com/problems/count-ways-to-build-good-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2466_example_1() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;

        let result = 8;

        assert_eq!(Solution::count_good_strings(low, high, zero, one), result);
    }

    #[test]
    #[ignore]
    fn test_2466_example_2() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;

        let result = 5;

        assert_eq!(Solution::count_good_strings(low, high, zero, one), result);
    }
}
