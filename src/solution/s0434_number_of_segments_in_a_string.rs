/**
 * [0434] Number of Segments in a String
 *
 * You are given a string s, return the number of segments in the string.
 * A segment is defined to be a contiguous sequence of non-space characters.
 *  
 * Example 1:
 *
 * Input: s = "Hello, my name is John"
 * Output: 5
 * Explanation: The five segments are ["Hello,", "my", "name", "is", "John"]
 *
 * Example 2:
 *
 * Input: s = "Hello"
 * Output: 1
 *
 * Example 3:
 *
 * Input: s = "love live! mu'sic forever"
 * Output: 4
 *
 * Example 4:
 *
 * Input: s = ""
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 300
 * 	s consists of lower-case and upper-case English letters, digits or one of the following characters "!@#$%^&amp;*()_+-=',.:".
 * 	The only space character in s is ' '.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-segments-in-a-string/
// discuss: https://leetcode.com/problems/number-of-segments-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().collect::<Vec<_>>().len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0434_example_1() {
        let s = "Hello".to_string();
        let result = 1;

        assert_eq!(Solution::count_segments(s), result);
    }

    #[test]
    fn test_0434_example_2() {
        let s = "Hello, my name is John".to_string();
        let result = 5;

        assert_eq!(Solution::count_segments(s), result);
    }

    #[test]
    fn test_0434_example_3() {
        let s = "love live! mu'sic forever".to_string();
        let result = 4;

        assert_eq!(Solution::count_segments(s), result);
    }

    #[test]
    fn test_0434_example_4() {
        let s = "".to_string();
        let result = 0;

        assert_eq!(Solution::count_segments(s), result);
    }
}
