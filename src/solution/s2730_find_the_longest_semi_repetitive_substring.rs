/**
 * [2730] Find the Longest Semi-Repetitive Substring
 *
 * You are given a digit string s that consists of digits from 0 to 9.
 * A string is called semi-repetitive if there is at most one adjacent pair of the same digit. For example, "0010", "002020", "0123", "2002", and "54944" are semi-repetitive while the following are not: "00101022" (adjacent same digit pairs are 00 and 22), and "1101234883" (adjacent same digit pairs are 11 and 88).
 * Return the length of the longest semi-repetitive substring of s.
 *  
 * Example 1:
 *
 * Input: s = "52233"
 * Output: 4
 * Explanation:
 * The longest semi-repetitive substring is "5223". Picking the whole string "52233" has two adjacent same digit pairs 22 and 33, but at most one is allowed.
 *
 * Example 2:
 *
 * Input: s = "5494"
 * Output: 4
 * Explanation:
 * s is a semi-repetitive string.
 *
 * Example 3:
 *
 * Input: s = "1111111"
 * Output: 2
 * Explanation:
 * The longest semi-repetitive substring is "11". Picking the substring "111" has two adjacent same digit pairs, but at most one is allowed.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 50
 * 	'0' <= s[i] <= '9'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/
// discuss: https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2730_example_1() {
        let s = "52233".to_string();

        let result = 4;

        assert_eq!(Solution::longest_semi_repetitive_substring(s), result);
    }

    #[test]
    #[ignore]
    fn test_2730_example_2() {
        let s = "5494".to_string();

        let result = 4;

        assert_eq!(Solution::longest_semi_repetitive_substring(s), result);
    }

    #[test]
    #[ignore]
    fn test_2730_example_3() {
        let s = "1111111".to_string();

        let result = 2;

        assert_eq!(Solution::longest_semi_repetitive_substring(s), result);
    }
}
