/**
 * [68] Text Justification
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 * Note:
 *
 * 	A word is defined as a character sequence consisting of non-space characters only.
 * 	Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 * 	The input array words contains at least one word.
 *
 *  
 * Example 1:
 *
 * Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 * Example 2:
 *
 * Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
 * Note that the second line is also left-justified becase it contains only one word.
 * Example 3:
 *
 * Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
 * Output:
 * [
 *   "Science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  Art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 300
 * 	1 <= words[i].length <= 20
 * 	words[i] consists of only English letters and symbols.
 * 	1 <= maxWidth <= 100
 * 	words[i].length <= maxWidth
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/text-justification/
// discuss: https://leetcode.com/problems/text-justification/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut line: Vec<_> = Vec::new();
        let mut len = 0;
        let max_width = max_width as usize;
        let mut result: Vec<_> = Vec::new();

        for word in words.iter() {
            if len + word.len() + line.len() <= max_width {
                line.push(word.to_string());
                len += word.len();
                continue;
            }
            if line.len() == 1 {
                result.push(format!("{0:1$}", line[0], max_width));
            } else {
                let temp = line.len() - 1;
                for i in 0..(max_width - len) {
                    line[i % temp].push(' ');
                }
                let temp = line.join("");
                result.push(temp);
            }
            line = vec![word.to_string()];
            len = word.len();
        }

        result.push(format!("{0:1$}", line.join(" "), max_width));
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0068_example_1() {
        let words = vec_string![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification."
        ];
        let max_width = 16;
        let result = vec_string!["This    is    an", "example  of text", "justification.  "];

        assert_eq!(Solution::full_justify(words, max_width), result);
    }

    #[test]
    fn test_0068_example_2() {
        let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
        let max_width = 16;
        let result = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];

        assert_eq!(Solution::full_justify(words, max_width), result);
    }

    #[test]
    fn test_0068_example_3() {
        let words = vec_string![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do"
        ];
        let max_width = 20;
        let result = vec_string![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ];

        assert_eq!(Solution::full_justify(words, max_width), result);
    }
}
