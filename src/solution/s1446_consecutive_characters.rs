/**
 * [1446] Consecutive Characters
 *
 * The power of the string is the maximum length of a non-empty substring that contains only one unique character.
 * Given a string s, return the power of s.
 *  
 * Example 1:
 *
 * Input: s = "leetcode"
 * Output: 2
 * Explanation: The substring "ee" is of length 2 with the character 'e' only.
 *
 * Example 2:
 *
 * Input: s = "abbcccddddeeeeedcba"
 * Output: 5
 * Explanation: The substring "eeeee" is of length 5 with the character 'e' only.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/consecutive-characters/
// discuss: https://leetcode.com/problems/consecutive-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_power(s: String) -> i32 {
        s.into_bytes()
            .windows(2)
            .scan(1, |s, x| {
                *s = if x[0] == x[1] { *s + 1 } else { 1 };
                Some(*s)
            })
            .max()
            .unwrap_or(1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1446_example_1() {
        let s = "leetcode".to_string();

        let result = 2;

        assert_eq!(Solution::max_power(s), result);
    }

    #[test]
    fn test_1446_example_2() {
        let s = "leabbcccddddeeeeedcbatcode".to_string();

        let result = 5;

        assert_eq!(Solution::max_power(s), result);
    }
}
