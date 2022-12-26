/**
 * [0925] Long Pressed Name
 *
 * Your friend is typing his name into a keyboard. Sometimes, when typing a character c, the key might get long pressed, and the character will be typed 1 or more times.
 * You examine the typed characters of the keyboard. Return True if it is possible that it was your friends name, with some characters (possibly none) being long pressed.
 *  
 * Example 1:
 *
 * Input: name = "alex", typed = "aaleex"
 * Output: true
 * Explanation: 'a' and 'e' in 'alex' were long pressed.
 *
 * Example 2:
 *
 * Input: name = "saeed", typed = "ssaaedd"
 * Output: false
 * Explanation: 'e' must have been pressed twice, but it was not in the typed output.
 *
 *  
 * Constraints:
 *
 * 	1 <= name.length, typed.length <= 1000
 * 	name and typed consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/long-pressed-name/
// discuss: https://leetcode.com/problems/long-pressed-name/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name = name.chars().collect::<Vec<char>>();
        let typed = typed.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        while j < typed.len() || i < name.len() {
            if i < name.len() && j < typed.len() && name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if j >= 1 && j < typed.len() && typed[j] == typed[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0925_example_1() {
        let name = "alex".to_string();
        let typed = "aaleex".to_string();
        let result = true;

        assert_eq!(Solution::is_long_pressed_name(name, typed), result);
    }

    #[test]
    fn test_0925_example_2() {
        let name = "saeed".to_string();
        let typed = "ssaaedd".to_string();
        let result = false;

        assert_eq!(Solution::is_long_pressed_name(name, typed), result);
    }
}
