/**
 * [0844] Backspace String Compare
 *
 * Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
 * Note that after backspacing an empty text, the text will continue empty.
 *  
 * Example 1:
 *
 * Input: s = "ab#c", t = "ad#c"
 * Output: true
 * Explanation: Both s and t become "ac".
 *
 * Example 2:
 *
 * Input: s = "ab##", t = "c#d#"
 * Output: true
 * Explanation: Both s and t become "".
 *
 * Example 3:
 *
 * Input: s = "a#c", t = "b"
 * Output: false
 * Explanation: s becomes "c" while t becomes "b".
 *
 *  
 * Constraints:
 *
 * 	<span>1 <= s.length, t.length <= 200</span>
 * 	<span>s and t only contain lowercase letters and '#' characters.</span>
 *
 *  
 * Follow up: Can you solve it in O(n) time and O(1) space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/backspace-string-compare/
// discuss: https://leetcode.com/problems/backspace-string-compare/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::simplify_str(s) == Self::simplify_str(t)
    }

    fn simplify_str(s: String) -> Vec<char> {
        let mut stack = Vec::new();
        s.chars().for_each(|c| match c {
            '#' => {
                stack.pop();
            }
            _ => stack.push(c),
        });
        stack
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0844_example_1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        let result = true;

        assert_eq!(Solution::backspace_compare(s, t), result);
    }

    #[test]
    fn test_0844_example_2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        let result = true;

        assert_eq!(Solution::backspace_compare(s, t), result);
    }

    #[test]
    fn test_0844_example_3() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        let result = false;

        assert_eq!(Solution::backspace_compare(s, t), result);
    }
}
