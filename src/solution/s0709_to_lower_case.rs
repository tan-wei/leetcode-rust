/**
 * [0709] To Lower Case
 *
 * Given a string s, return the string after replacing every uppercase letter with the same lowercase letter.
 *  
 * Example 1:
 *
 * Input: s = "Hello"
 * Output: "hello"
 *
 * Example 2:
 *
 * Input: s = "here"
 * Output: "here"
 *
 * Example 3:
 *
 * Input: s = "LOVELY"
 * Output: "lovely"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of printable ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/to-lower-case/
// discuss: https://leetcode.com/problems/to-lower-case/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_ascii_lowercase()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0709_example_1() {
        let s = "Hello".to_string();
        let result = "hello".to_string();

        assert_eq!(Solution::to_lower_case(s), result);
    }

    #[test]
    fn test_0709_example_2() {
        let s = "here".to_string();
        let result = "here".to_string();

        assert_eq!(Solution::to_lower_case(s), result);
    }

    #[test]
    fn test_0709_example_3() {
        let s = "LOVELY".to_string();
        let result = "lovely".to_string();

        assert_eq!(Solution::to_lower_case(s), result);
    }
}
