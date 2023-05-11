/**
 * [1081] Smallest Subsequence of Distinct Characters
 *
 * Given a string s, return the <span data-keyword="lexicographically-smaller-string">lexicographically smallest</span> <span data-keyword="subsequence-string">subsequence</span> of s that contains all the distinct characters of s exactly once.
 *  
 * Example 1:
 *
 * Input: s = "bcabc"
 * Output: "abc"
 *
 * Example 2:
 *
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters.
 *
 *  
 * Note: This question is the same as 316: <a href="https://leetcode.com/problems/remove-duplicate-letters/" target="_blank">https://leetcode.com/problems/remove-duplicate-letters/</a>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
// discuss: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - 'a' as u8) as usize] += 1;
        }

        let mut flag = vec![0; 26];
        let mut stack = std::collections::VecDeque::<char>::new();

        for c in s.chars() {
            count[(c as u8 - 'a' as u8) as usize] -= 1;
            if flag[(c as u8 - 'a' as u8) as usize] == 1 {
                continue;
            }

            while let Some(cc) = stack.back() {
                let top = *cc;
                if count[(top as u8 - 'a' as u8) as usize] == 0 {
                    break;
                }
                if top < c {
                    break;
                }
                stack.pop_back();
                flag[(top as u8 - 'a' as u8) as usize] = 0;
            }

            stack.push_back(c);
            flag[(c as u8 - 'a' as u8) as usize] = 1;
        }

        stack.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1081_example_1() {
        let s = "bcabc".to_string();
        let result = "abc".to_string();

        assert_eq!(Solution::smallest_subsequence(s), result);
    }

    #[test]
    fn test_1081_example_2() {
        let s = "cbacdcbc".to_string();
        let result = "acdb".to_string();

        assert_eq!(Solution::smallest_subsequence(s), result);
    }
}
