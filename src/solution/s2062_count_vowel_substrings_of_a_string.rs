/**
 * [2062] Count Vowel Substrings of a String
 *
 * A substring is a contiguous (non-empty) sequence of characters within a string.
 * A vowel substring is a substring that only consists of vowels ('a', 'e', 'i', 'o', and 'u') and has all five vowels present in it.
 * Given a string word, return the number of vowel substrings in word.
 *  
 * Example 1:
 *
 * Input: word = "aeiouu"
 * Output: 2
 * Explanation: The vowel substrings of word are as follows (underlined):
 * - "<u>aeiou</u>u"
 * - "<u>aeiouu</u>"
 *
 * Example 2:
 *
 * Input: word = "unicornarihan"
 * Output: 0
 * Explanation: Not all 5 vowels are present, so there are no vowel substrings.
 *
 * Example 3:
 *
 * Input: word = "cuaieuouac"
 * Output: 7
 * Explanation: The vowel substrings of word are as follows (underlined):
 * - "c<u>uaieuo</u>uac"
 * - "c<u>uaieuou</u>ac"
 * - "c<u>uaieuoua</u>c"
 * - "cu<u>aieuo</u>uac"
 * - "cu<u>aieuou</u>ac"
 * - "cu<u>aieuoua</u>c"
 * - "cua<u>ieuoua</u>c"
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 100
 * 	word consists of lowercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-vowel-substrings-of-a-string/
// discuss: https://leetcode.com/problems/count-vowel-substrings-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut count = 0;
        let s = word.as_bytes();

        for i in 0..s.len() {
            let mut st = std::collections::HashSet::new();
            for j in i..s.len() {
                if !"aeiou".contains(s[j] as char) {
                    break;
                } else {
                    st.insert(s[j]);
                    if st.len() == 5 {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2062_example_1() {
        let word = "aeiouu".to_string();

        let result = 2;

        assert_eq!(Solution::count_vowel_substrings(word), result);
    }

    #[test]
    fn test_2062_example_2() {
        let word = "unicornarihan".to_string();

        let result = 0;

        assert_eq!(Solution::count_vowel_substrings(word), result);
    }

    #[test]
    fn test_2062_example_3() {
        let word = "cuaieuouac".to_string();

        let result = 7;

        assert_eq!(Solution::count_vowel_substrings(word), result);
    }
}
