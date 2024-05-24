/**
 * [1614] Maximum Nesting Depth of the Parentheses
 *
 * Given a valid parentheses string s, return the nesting depth of s. The nesting depth is the maximum number of nested parentheses.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "(1+(2*3)+((8)/4))+1"</span>
 * Output: <span class="example-io">3</span>
 * Explanation:
 * Digit 8 is inside of 3 nested parentheses in the string.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = "(1)+((2))+(((3)))"</span>
 * Output: <span class="example-io">3</span>
 * Explanation:
 * Digit 3 is inside of 3 nested parentheses in the string.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">s = "()(())((()()))"</span>
 * Output: <span class="example-io">3</span>
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of digits 0-9 and characters '+', '-', '*', '/', '(', and ')'.
 * 	It is guaranteed that parentheses expression s is a VPS.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
// discuss: https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars()
            .scan(0, |mut state, c| {
                match c {
                    '(' => *state += 1,
                    ')' => *state -= 1,
                    _ => (),
                }
                Some(*state)
            })
            .max()
            .unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1614_example_1() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();

        let result = 3;

        assert_eq!(Solution::max_depth(s), result);
    }

    #[test]
    fn test_1614_example_2() {
        let s = "(1)+((2))+(((3)))".to_string();

        let result = 3;

        assert_eq!(Solution::max_depth(s), result);
    }

    #[test]
    fn test_1614_example_3() {
        let s = "()(())((()()))".to_string();

        let result = 3;

        assert_eq!(Solution::max_depth(s), result);
    }
}
