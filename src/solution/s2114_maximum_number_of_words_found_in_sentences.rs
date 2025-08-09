/**
 * [2114] Maximum Number of Words Found in Sentences
 *
 * A sentence is a list of words that are separated by a single space with no leading or trailing spaces.
 * You are given an array of strings sentences, where each sentences[i] represents a single sentence.
 * Return the maximum number of words that appear in a single sentence.
 *  
 * Example 1:
 *
 * Input: sentences = ["alice and bob love leetcode", "i think so too", <u>"this is great thanks very much"</u>]
 * Output: 6
 * Explanation:
 * - The first sentence, "alice and bob love leetcode", has 5 words in total.
 * - The second sentence, "i think so too", has 4 words in total.
 * - The third sentence, "this is great thanks very much", has 6 words in total.
 * Thus, the maximum number of words in a single sentence comes from the third sentence, which has 6 words.
 *
 * Example 2:
 *
 * Input: sentences = ["please wait", <u>"continue to fight"</u>, <u>"continue to win"</u>]
 * Output: 3
 * Explanation: It is possible that multiple sentences contain the same number of words.
 * In this example, the second and third sentences (underlined) have the same number of words.
 *
 *  
 * Constraints:
 *
 * 	1 <= sentences.length <= 100
 * 	1 <= sentences[i].length <= 100
 * 	sentences[i] consists only of lowercase English letters and ' ' only.
 * 	sentences[i] does not have leading or trailing spaces.
 * 	All the words in sentences[i] are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
// discuss: https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.matches(' ').count() + 1)
            .max()
            .unwrap() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2114_example_1() {
        let sentences = vec_string![
            "alice and bob love leetcode",
            "i think so too",
            "this is great thanks very much"
        ];

        let result = 6;

        assert_eq!(Solution::most_words_found(sentences), result);
    }

    #[test]
    fn test_2114_example_2() {
        let sentences = vec_string!["please wait", "continue to fight", "continue to win"];

        let result = 3;

        assert_eq!(Solution::most_words_found(sentences), result);
    }
}
