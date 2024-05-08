/**
 * [1592] Rearrange Spaces Between Words
 *
 * You are given a string text of words that are placed among some number of spaces. Each word consists of one or more lowercase English letters and are separated by at least one space. It's guaranteed that text contains at least one word.
 * Rearrange the spaces so that there is an equal number of spaces between every pair of adjacent words and that number is maximized. If you cannot redistribute all the spaces equally, place the extra spaces at the end, meaning the returned string should be the same length as text.
 * Return the string after rearranging the spaces.
 *  
 * Example 1:
 *
 * Input: text = "  this   is  a sentence "
 * Output: "this   is   a   sentence"
 * Explanation: There are a total of 9 spaces and 4 words. We can evenly divide the 9 spaces between the words: 9 / (4-1) = 3 spaces.
 *
 * Example 2:
 *
 * Input: text = " practice   makes   perfect"
 * Output: "practice   makes   perfect "
 * Explanation: There are a total of 7 spaces and 3 words. 7 / (3-1) = 3 spaces plus 1 extra space. We place this extra space at the end of the string.
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 100
 * 	text consists of lowercase English letters and ' '.
 * 	text contains at least one word.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rearrange-spaces-between-words/
// discuss: https://leetcode.com/problems/rearrange-spaces-between-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let count_whitespace = text.chars().filter(|x| *x == ' ').count();
        if count_whitespace == 0 {
            return text;
        }
        let div_mod = |x, y| (x / y, x % y);
        let words = text.split_whitespace().collect::<Vec<_>>();

        if words.len() == 1 {
            words[0].to_owned() + " ".repeat(count_whitespace).as_str()
        } else {
            let (div_sep, mod_sep) = div_mod(count_whitespace, words.len() - 1);
            words.join(" ".repeat(div_sep).as_str()) + " ".repeat(mod_sep).as_str()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1592_example_1() {
        let text = "  this   is  a sentence ".to_string();

        let result = "this   is   a   sentence".to_string();

        assert_eq!(Solution::reorder_spaces(text), result);
    }

    #[test]
    fn test_1592_example_2() {
        let text = " practice   makes   perfect".to_string();

        let result = "practice   makes   perfect ".to_string();

        assert_eq!(Solution::reorder_spaces(text), result);
    }
}
