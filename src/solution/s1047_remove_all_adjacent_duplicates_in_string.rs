/**
 * [1047] Remove All Adjacent Duplicates In String
 *
 * You are given a string s consisting of lowercase English letters. A duplicate removal consists of choosing two adjacent and equal letters and removing them.
 * We repeatedly make duplicate removals on s until we no longer can.
 * Return the final string after all such duplicate removals have been made. It can be proven that the answer is unique.
 *  
 * Example 1:
 *
 * Input: s = "abbaca"
 * Output: "ca"
 * Explanation:
 * For example, in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.  The result of this move is that the string is "aaca", of which only "aa" is possible, so the final string is "ca".
 *
 * Example 2:
 *
 * Input: s = "azxxzy"
 * Output: "ay"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
// discuss: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
            .fold(vec![], |mut stack, c| {
                if stack.last() == Some(&c) {
                    stack.pop();
                } else {
                    stack.push(c);
                }
                stack
            })
            .iter()
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1047_example_1() {
        let s = "abbaca".to_string();
        let result = "ca".to_string();

        assert_eq!(Solution::remove_duplicates(s), result);
    }

    #[test]
    fn test_1047_example_2() {
        let s = "azxxzy".to_string();
        let result = "ay".to_string();

        assert_eq!(Solution::remove_duplicates(s), result);
    }
}
