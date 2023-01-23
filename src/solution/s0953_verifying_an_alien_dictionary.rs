/**
 * [0953] Verifying an Alien Dictionary
 *
 * In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
 * Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.
 *  
 * Example 1:
 *
 * Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
 * Output: true
 * Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
 *
 * Example 2:
 *
 * Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
 * Output: false
 * Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
 *
 * Example 3:
 *
 * Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
 * Output: false
 * Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '&empty;', where '&empty;' is defined as the blank character which is less than any other character (<a href="https://en.wikipedia.org/wiki/Lexicographical_order" target="_blank">More info</a>).
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 20
 * 	order.length == 26
 * 	All characters in words[i] and order are English lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/verifying-an-alien-dictionary/
// discuss: https://leetcode.com/problems/verifying-an-alien-dictionary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order = order
            .as_bytes()
            .iter()
            .enumerate()
            .fold([0; 26], |mut acc, (i, b)| {
                acc[(b - b'a') as usize] = i;
                acc
            });
        words
            .iter()
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .map(|&b| order[(b - b'a') as usize])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .windows(2)
            .all(|words| words[0] <= words[1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0953_example_1() {
        let words = vec_string!["hello", "leetcode"];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let result = true;

        assert_eq!(Solution::is_alien_sorted(words, order), result);
    }

    #[test]
    fn test_0953_example_2() {
        let words = vec_string!["word", "world", "row"];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        let result = false;

        assert_eq!(Solution::is_alien_sorted(words, order), result);
    }

    #[test]
    fn test_0953_example_3() {
        let words = vec_string!["apple", "app"];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = false;

        assert_eq!(Solution::is_alien_sorted(words, order), result);
    }
}
