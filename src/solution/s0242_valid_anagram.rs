/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 *  
 * Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 5 * 10^4
 * 	s and t consist of lowercase English letters.
 *
 *  
 * Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.bytes().collect::<Vec<u8>>();
        let mut t = t.bytes().collect::<Vec<u8>>();

        s.sort_unstable();
        t.sort_unstable();

        s == t
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0242_example_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let result = true;

        assert_eq!(Solution::is_anagram(s, t), result);
    }

    #[test]
    fn test_0242_example_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let result = false;

        assert_eq!(Solution::is_anagram(s, t), result);
    }
}
