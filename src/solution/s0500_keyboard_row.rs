/**
 * [0500] Keyboard Row
 *
 * Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
 * In the American keyboard:
 *
 * 	the first row consists of the characters "qwertyuiop",
 * 	the second row consists of the characters "asdfghjkl", and
 * 	the third row consists of the characters "zxcvbnm".
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/keyboard.png" style="width: 800px; max-width: 600px; height: 267px;" />
 *  
 * Example 1:
 *
 * Input: words = ["Hello","Alaska","Dad","Peace"]
 * Output: ["Alaska","Dad"]
 *
 * Example 2:
 *
 * Input: words = ["omk"]
 * Output: []
 *
 * Example 3:
 *
 * Input: words = ["adsdf","sfd"]
 * Output: ["adsdf","sfd"]
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 20
 * 	1 <= words[i].length <= 100
 * 	words[i] consists of English letters (both lowercase and uppercase).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/keyboard-row/
// discuss: https://leetcode.com/problems/keyboard-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    const KEYBOARD_ROWS: [&'static str; 3] = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

    pub fn find_words(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .filter(|s| {
                Self::KEYBOARD_ROWS.iter().fold(false, |b, &row| {
                    b || s.to_lowercase().chars().all(|c| row.contains(c))
                })
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0500_example_1() {
        let words = vec_string!["Hello", "Alaska", "Dad", "Peace"];
        let result = vec_string!["Alaska", "Dad"];

        assert_eq!(Solution::find_words(words), result);
    }

    #[test]
    fn test_0500_example_2() {
        let words = vec_string!["omk"];
        let result: Vec<String> = vec_string![];

        assert_eq!(Solution::find_words(words), result);
    }

    #[test]
    fn test_0500_example_3() {
        let words = vec_string!["adsdf", "sfd"];
        let result = vec_string!["adsdf", "sfd"];

        assert_eq!(Solution::find_words(words), result);
    }
}
