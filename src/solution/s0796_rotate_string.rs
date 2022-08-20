/**
 * [0796] Rotate String
 *
 * Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.
 * A shift on s consists of moving the leftmost character of s to the rightmost position.
 *
 * 	For example, if s = "abcde", then it will be "bcdea" after one shift.
 *
 *  
 * Example 1:
 * Input: s = "abcde", goal = "cdeab"
 * Output: true
 * Example 2:
 * Input: s = "abcde", goal = "abced"
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= s.length, goal.length <= 100
 * 	s and goal consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-string/
// discuss: https://leetcode.com/problems/rotate-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && goal.repeat(2).contains(s.as_str())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0796_example_1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        let result = true;

        assert_eq!(Solution::rotate_string(s, goal), result);
    }

    #[test]
    fn test_0796_example_2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        let result = false;

        assert_eq!(Solution::rotate_string(s, goal), result);
    }
}
