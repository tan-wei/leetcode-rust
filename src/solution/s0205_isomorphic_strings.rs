/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings s and t are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
 *  
 * Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 * Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 * Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	t.length == s.length
 * 	s and t consist of any valid ascii character.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/isomorphic-strings/
// discuss: https://leetcode.com/problems/isomorphic-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut assign_s_t = std::collections::BTreeMap::new();
        let mut assign_t_s = std::collections::BTreeMap::new();
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if *assign_s_t.entry(ch_s).or_insert(ch_t) != ch_t {
                return false;
            }
            if *assign_t_s.entry(ch_t).or_insert(ch_s) != ch_s {
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
    fn test_0205_example_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let result = true;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }

    #[test]
    fn test_0205_example_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let result = false;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }

    #[test]
    fn test_0205_example_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        let result = true;

        assert_eq!(Solution::is_isomorphic(s, t), result);
    }
}
