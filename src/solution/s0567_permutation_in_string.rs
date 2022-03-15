/**
 * [0567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
 * In other words, return true if one of s1's permutations is the substring of s2.
 *  
 * Example 1:
 *
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 * Example 2:
 *
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 10^4
 * 	s1 and s2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-in-string/
// discuss: https://leetcode.com/problems/permutation-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/permutation-in-string/discuss/638658/Rust-solution
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut d1 = [0; 26];
        let mut d2 = [0; 26];

        for c in s1.chars() {
            d1[(c as u8 - b'a') as usize] += 1;
        }
        let s = s2.as_bytes();

        for (i, c) in s.iter().enumerate() {
            d2[(c - b'a') as usize] += 1;

            if i >= s1.len() {
                d2[(s[i - s1.len()] - b'a') as usize] -= 1;
            }

            if d1 == d2 {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0567_example_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let result = true;

        assert_eq!(Solution::check_inclusion(s1, s2), result);
    }

    #[test]
    fn test_0567_example_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let result = false;

        assert_eq!(Solution::check_inclusion(s1, s2), result);
    }
}
