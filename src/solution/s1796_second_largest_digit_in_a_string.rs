/**
 * [1796] Second Largest Digit in a String
 *
 * Given an alphanumeric string s, return the second largest numerical digit that appears in s, or -1 if it does not exist.
 * An alphanumeric string is a string consisting of lowercase English letters and digits.
 *  
 * Example 1:
 *
 * Input: s = "dfa12321afd"
 * Output: 2
 * Explanation: The digits that appear in s are [1, 2, 3]. The second largest digit is 2.
 *
 * Example 2:
 *
 * Input: s = "abc1111"
 * Output: -1
 * Explanation: The digits that appear in s are [1]. There is no second largest digit.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of only lowercase English letters and digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/second-largest-digit-in-a-string/
// discuss: https://leetcode.com/problems/second-largest-digit-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        s.bytes()
            .filter_map(|b| b.is_ascii_digit().then_some((b - b'0') as i32))
            .fold((-1, -1), |(p, u), d| {
                if d > u {
                    (u, d)
                } else if d > p && d < u {
                    (d, u)
                } else {
                    (p, u)
                }
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1796_example_1() {
        let s = "dfa12321afd".to_string();

        let result = 2;

        assert_eq!(Solution::second_highest(s), result);
    }

    #[test]
    fn test_1796_example_2() {
        let s = "abc1111".to_string();

        let result = -1;

        assert_eq!(Solution::second_highest(s), result);
    }
}
