/**
 * [345] Reverse Vowels of a String
 *
 * Given a string s, reverse only all the vowels in the string and return it.
 * The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both cases.
 *  
 * Example 1:
 * Input: s = "hello"
 * Output: "holle"
 * Example 2:
 * Input: s = "leetcode"
 * Output: "leotcede"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 3 * 10^5
 * 	s consist of printable ASCII characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-vowels-of-a-string/
// discuss: https://leetcode.com/problems/reverse-vowels-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        fn is_vowel(c: char) -> bool {
            c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        }

        let mut i = 0;
        let mut j = s.len() - 1;
        let mut chars = s.chars().collect::<Vec<char>>();
        while i < j {
            if !is_vowel(chars[i].to_ascii_lowercase()) {
                i += 1;
            } else {
                if !is_vowel(chars[j].to_ascii_lowercase()) {
                    j -= 1;
                } else {
                    chars.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
        }

        chars.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0345_example_1() {
        let s = "hello".to_string();
        let result = "holle".to_string();

        assert_eq!(Solution::reverse_vowels(s), result);
    }

    #[test]
    fn test_0345_example_2() {
        let s = "leetcode".to_string();
        let result = "leotcede".to_string();

        assert_eq!(Solution::reverse_vowels(s), result);
    }
}
