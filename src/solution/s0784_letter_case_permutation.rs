/**
 * [0784] Letter Case Permutation
 *
 * Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.
 * Return a list of all possible strings we could create. Return the output in any order.
 *  
 * Example 1:
 *
 * Input: s = "a1b2"
 * Output: ["a1b2","a1B2","A1b2","A1B2"]
 *
 * Example 2:
 *
 * Input: s = "3z4"
 * Output: ["3z4","3Z4"]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 12
 * 	s consists of lowercase English letters, uppercase English letters, and digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-case-permutation/
// discuss: https://leetcode.com/problems/letter-case-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut vd = std::collections::VecDeque::new();
        vd.push_back(s.clone());
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_alphabetic() {
                (0..vd.len()).for_each(|_| {
                    if let Some(front) = vd.pop_front() {
                        vd.push_back(
                            front[0..i].to_string()
                                + &c.to_ascii_uppercase().to_string()
                                + &front[i + 1..],
                        );
                        vd.push_back(
                            front[0..i].to_string()
                                + &c.to_ascii_lowercase().to_string()
                                + &front[i + 1..],
                        );
                    }
                });
            }
        }
        vd.into_iter().collect()
    }

    pub fn letter_case_permutation_v2(s: String) -> Vec<String> {
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut answer = Vec::new();
        Self::dfs_helper(&mut chars, &mut answer, 0);
        answer
    }

    fn dfs_helper(chars: &mut [char], answer: &mut Vec<String>, i: usize) {
        if i == chars.len() {
            answer.push(chars.iter().collect());
        } else {
            Self::dfs_helper(chars, answer, i + 1);
            if chars[i].is_alphabetic() {
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
                Self::dfs_helper(chars, answer, i + 1);
                chars[i] = ((chars[i] as u8) ^ (1 << 5)) as char;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0784_example_1() {
        let s = "a1b2".to_string();
        let result = vec_string!["a1b2", "a1B2", "A1b2", "A1B2"];

        assert_eq_sorted!(Solution::letter_case_permutation(s), result);

        let s = "a1b2".to_string();
        let result = vec_string!["a1b2", "a1B2", "A1b2", "A1B2"];

        assert_eq_sorted!(Solution::letter_case_permutation_v2(s), result);
    }

    #[test]
    fn test_0784_example_2() {
        let s = "3z4".to_string();
        let result = vec_string!["3z4", "3Z4"];

        assert_eq_sorted!(Solution::letter_case_permutation(s), result);

        let s = "3z4".to_string();
        let result = vec_string!["3z4", "3Z4"];

        assert_eq_sorted!(Solution::letter_case_permutation_v2(s), result);
    }
}
