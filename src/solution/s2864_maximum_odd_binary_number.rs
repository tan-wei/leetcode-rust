/**
 * [2864] Maximum Odd Binary Number
 *
 * You are given a binary string s that contains at least one '1'.
 * You have to rearrange the bits in such a way that the resulting binary number is the maximum odd binary number that can be created from this combination.
 * Return a string representing the maximum odd binary number that can be created from the given combination.
 * Note that the resulting string can have leading zeros.
 *  
 * Example 1:
 *
 * Input: s = "010"
 * Output: "001"
 * Explanation: Because there is just one '1', it must be in the last position. So the answer is "001".
 *
 * Example 2:
 *
 * Input: s = "0101"
 * Output: "1001"
 * Explanation: One of the '1's must be in the last position. The maximum number that can be made with the remaining digits is "100". So the answer is "1001".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists only of '0' and '1'.
 * 	s contains at least one '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-odd-binary-number/
// discuss: https://leetcode.com/problems/maximum-odd-binary-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-odd-binary-number/solutions/4805921/rust-0ms-203mb-one-liner-solution-by-ron-eayb/
    pub fn maximum_odd_binary_number(s: String) -> String {
        format!(
            "{x:1>y$}{x:0>z$}1",
            x = "",
            y = s.chars().filter(|c| c == &'1').count() - 1,
            z = s.chars().filter(|c| c == &'0').count()
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2864_example_1() {
        let s = "010".to_string();

        let result = "001".to_string();

        assert_eq!(Solution::maximum_odd_binary_number(s), result);
    }

    #[test]
    fn test_2864_example_2() {
        let s = "0101".to_string();

        let result = "1001".to_string();

        assert_eq!(Solution::maximum_odd_binary_number(s), result);
    }
}
