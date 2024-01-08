/**
 * [1451] Rearrange Words in a Sentence
 *
 * Given a sentence text (A sentence is a string of space-separated words) in the following format:
 *
 * 	First letter is in upper case.
 * 	Each word in text are separated by a single space.
 *
 * Your task is to rearrange the words in text such that all words are rearranged in an increasing order of their lengths. If two words have the same length, arrange them in their original order.
 * Return the new text following the format shown above.
 *  
 * Example 1:
 *
 * Input: text = "Leetcode is cool"
 * Output: "Is cool leetcode"
 * Explanation: There are 3 words, "Leetcode" of length 8, "is" of length 2 and "cool" of length 4.
 * Output is ordered by length and the new first word starts with capital letter.
 *
 * Example 2:
 *
 * Input: text = "Keep calm and code on"
 * Output: "On and keep calm code"
 * Explanation: Output is ordered as follows:
 * "On" 2 letters.
 * "and" 3 letters.
 * "keep" 4 letters in case of tie order by position in original text.
 * "calm" 4 letters.
 * "code" 4 letters.
 *
 * Example 3:
 *
 * Input: text = "To be or not to be"
 * Output: "To be or to be not"
 *
 *  
 * Constraints:
 *
 * 	text begins with a capital letter and then contains lowercase letters and single space between words.
 * 	1 <= text.length <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rearrange-words-in-a-sentence/
// discuss: https://leetcode.com/problems/rearrange-words-in-a-sentence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn arrange_words(text: String) -> String {
        let words: Vec<_> = text.split(' ').collect();
        let mut words = words
            .into_iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>();
        words.sort_by_key(|w| w.len());
        words[0] = Self::upper_first_char(&words[0]);
        words.join(" ")
    }

    fn upper_first_char(s: &str) -> String {
        let mut chars = s.chars();
        let c = chars.next().unwrap().to_uppercase().to_string();
        let s: String = chars.collect();
        c + &s
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1451_example_1() {
        let text = "Leetcode is cool".to_string();

        let result = "Is cool leetcode".to_string();

        assert_eq!(Solution::arrange_words(text), result);
    }

    #[test]
    fn test_1451_example_2() {
        let text = "Keep calm and code on".to_string();

        let result = "On and keep calm code".to_string();

        assert_eq!(Solution::arrange_words(text), result);
    }

    #[test]
    fn test_1451_example_3() {
        let text = "To be or not to be".to_string();

        let result = "To be or to be not".to_string();

        assert_eq!(Solution::arrange_words(text), result);
    }
}
