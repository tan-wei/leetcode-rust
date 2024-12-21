/**
 * [1813] Sentence Similarity III
 *
 * You are given two strings sentence1 and sentence2, each representing a sentence composed of words. A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of only uppercase and lowercase English characters.
 * Two sentences s1 and s2 are considered similar if it is possible to insert an arbitrary sentence (possibly empty) inside one of these sentences such that the two sentences become equal. Note that the inserted sentence must be separated from existing words by spaces.
 * For example,
 *
 * 	s1 = "Hello Jane" and s2 = "Hello my name is Jane" can be made equal by inserting "my name is" between "Hello"<font face="monospace"> </font>and "Jane"<font face="monospace"> in s1.</font>
 * 	<font face="monospace">s1 = "Frog cool" </font>and<font face="monospace"> s2 = "Frogs are cool" </font>are not similar, since although there is a sentence "s are" inserted into s1, it is not separated from "Frog" by a space.
 *
 * Given two sentences sentence1 and sentence2, return true if sentence1 and sentence2 are similar. Otherwise, return false.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">sentence1 = "My name is Haley", sentence2 = "My Haley"</span>
 * Output: <span class="example-io">true</span>
 * Explanation:
 * sentence2 can be turned to sentence1 by inserting "name is" between "My" and "Haley".
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">sentence1 = "of", sentence2 = "A lot of words"</span>
 * Output: <span class="example-io">false</span>
 * Explanation:
 * No single sentence can be inserted inside one of the sentences to make it equal to the other.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">sentence1 = "Eating right now", sentence2 = "Eating"</span>
 * Output: <span class="example-io">true</span>
 * Explanation:
 * sentence2 can be turned to sentence1 by inserting "right now" at the end of the sentence.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= sentence1.length, sentence2.length <= 100
 * 	sentence1 and sentence2 consist of lowercase and uppercase English letters and spaces.
 * 	The words in sentence1 and sentence2 are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sentence-similarity-iii/
// discuss: https://leetcode.com/problems/sentence-similarity-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut iter1 = sentence1.split(' ').peekable();
        let mut iter2 = sentence2.split(' ').peekable();

        while let (Some(s1), Some(s2)) = (iter1.peek(), iter2.peek()) {
            if s1 != s2 {
                break;
            }
            iter1.next();
            iter2.next();
        }

        while let (Some(s1), Some(s2)) = (iter1.next_back(), iter2.next_back()) {
            if s1 != s2 {
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
    fn test_1813_example_1() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();

        let result = true;

        assert_eq!(
            Solution::are_sentences_similar(sentence1, sentence2),
            result
        );
    }

    #[test]
    fn test_1813_example_2() {
        let sentence1 = "of".to_string();
        let sentence2 = "A lot of words".to_string();

        let result = false;

        assert_eq!(
            Solution::are_sentences_similar(sentence1, sentence2),
            result
        );
    }

    #[test]
    fn test_1813_example_3() {
        let sentence1 = "Eating right now".to_string();
        let sentence2 = "Eating".to_string();

        let result = true;

        assert_eq!(
            Solution::are_sentences_similar(sentence1, sentence2),
            result
        );
    }
}
