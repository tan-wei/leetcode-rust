/**
 * [1541] Minimum Insertions to Balance a Parentheses String
 *
 * Given a parentheses string s containing only the characters '(' and ')'. A parentheses string is balanced if:
 *
 * 	Any left parenthesis '(' must have a corresponding two consecutive right parenthesis '))'.
 * 	Left parenthesis '(' must go before the corresponding two consecutive right parenthesis '))'.
 *
 * In other words, we treat '(' as an opening parenthesis and '))' as a closing parenthesis.
 *
 * 	For example, "())", "())(())))" and "(())())))" are balanced, ")()", "()))" and "(()))" are not balanced.
 *
 * You can insert the characters '(' and ')' at any position of the string to balance it if needed.
 * Return the minimum number of insertions needed to make s balanced.
 *  
 * Example 1:
 *
 * Input: s = "(()))"
 * Output: 1
 * Explanation: The second '(' has two matching '))', but the first '(' has only ')' matching. We need to add one more ')' at the end of the string to be "(())))" which is balanced.
 *
 * Example 2:
 *
 * Input: s = "())"
 * Output: 0
 * Explanation: The string is already balanced.
 *
 * Example 3:
 *
 * Input: s = "))())("
 * Output: 3
 * Explanation: Add '(' to match the first '))', Add '))' to match the last '('.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of '(' and ')' only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/
// discuss: https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let (mut result, mut right) = (0, 0);
        let s = s.as_bytes();
        for &c in s.iter() {
            if c == b'(' {
                if right % 2 > 0 {
                    right -= 1;
                    result += 1;
                }
                right += 2;
            } else {
                right -= 1;
                if right < 0 {
                    right += 2;
                    result += 1;
                }
            }
        }
        right + result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1541_example_1() {
        let s = "(()))".to_string();

        let result = 1;

        assert_eq!(Solution::min_insertions(s), result);
    }

    #[test]
    fn test_1541_example_2() {
        let s = "())".to_string();

        let result = 0;

        assert_eq!(Solution::min_insertions(s), result);
    }

    #[test]
    fn test_1541_example_3() {
        let s = "))())(".to_string();

        let result = 3;

        assert_eq!(Solution::min_insertions(s), result);
    }
}
