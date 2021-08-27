/**
 * [301] Remove Invalid Parentheses
 *
 * Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
 * Return all the possible results. You may return the answer in any order.
 *  
 * Example 1:
 *
 * Input: s = "()())()"
 * Output: ["(())()","()()()"]
 *
 * Example 2:
 *
 * Input: s = "(a)())()"
 * Output: ["(a())()","(a)()()"]
 *
 * Example 3:
 *
 * Input: s = ")("
 * Output: [""]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 25
 * 	s consists of lowercase English letters and parentheses '(' and ')'.
 * 	There will be at most 20 parentheses in s.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-invalid-parentheses/
// discuss: https://leetcode.com/problems/remove-invalid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/remove-invalid-parentheses/discuss/864630/Rust-translated-0ms-100
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        use std::iter::FromIterator;

        fn remove(s: &[char], ans: &mut Vec<String>, last_i: i32, last_j: i32, par: &[char]) {
            let mut stack = 0;
            for i in last_i..s.len() as i32 {
                if s[i as usize] == par[0] {
                    stack += 1;
                }
                if s[i as usize] == par[1] {
                    stack -= 1;
                }
                if stack >= 0 {
                    continue;
                }
                for j in last_j..=i {
                    if s[j as usize] == par[1] && (j == last_j || s[j as usize - 1] != par[1]) {
                        let mut s2 = Vec::new();
                        for k in 0..j as usize {
                            s2.push(s[k]);
                        }
                        for k in j as usize + 1..s.len() {
                            s2.push(s[k]);
                        }
                        remove(&s2, ans, i, j, par);
                    }
                }
                return;
            }
            let mut reversed = s.to_vec();
            reversed.reverse();
            if par[0] == '(' {
                remove(&reversed, ans, 0, 0, &[')', '(']);
            } else {
                ans.push(String::from_iter(reversed));
            }
        }
        let mut result = vec![];
        remove(
            &s.chars().collect::<Vec<char>>(),
            &mut result,
            0,
            0,
            &['(', ')'],
        );
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0301_example_1() {
        let s = "()())()".to_string();
        let result = vec_string!["(())()", "()()()"];

        assert_eq_sorted!(Solution::remove_invalid_parentheses(s), result);
    }

    #[test]
    fn test_0301_example_2() {
        let s = "(a)())()".to_string();
        let result = vec_string!["(a())()", "(a)()()"];

        assert_eq_sorted!(Solution::remove_invalid_parentheses(s), result);
    }

    #[test]
    fn test_0301_example_3() {
        let s = ")(".to_string();
        let result = vec_string![""];

        assert_eq_sorted!(Solution::remove_invalid_parentheses(s), result);
    }
}
