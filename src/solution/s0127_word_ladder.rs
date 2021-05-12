/**
 * [127] Word Ladder
 *
 * A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
 *
 * 	Every adjacent pair of words differs by a single letter.
 * 	Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
 * 	sk == endWord
 *
 * Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
 *  
 * Example 1:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
 * Output: 5
 * Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
 *
 * Example 2:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
 * Output: 0
 * Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
 *
 *  
 * Constraints:
 *
 * 	1 <= beginWord.length <= 10
 * 	endWord.length == beginWord.length
 * 	1 <= wordList.length <= 5000
 * 	wordList[i].length == beginWord.length
 * 	beginWord, endWord, and wordList[i] consist of lowercase English letters.
 * 	beginWord != endWord
 * 	All the words in wordList are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-ladder/
// discuss: https://leetcode.com/problems/word-ladder/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = HashSet::new();

        for word in word_list {
            words.insert(word);
        }
        if !words.contains(&end_word) {
            return 0;
        }

        let mut distance: i32 = 1;
        let mut q = vec![begin_word.clone()];
        let mut reached = HashSet::new();
        reached.insert(begin_word);
        let letters = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

        while q.len() > 0 {
            let q_size = q.len();
            for _i in 0..q_size {
                let curr_word = q.remove(0);
                for j in 0..curr_word.len() {
                    for k in &letters {
                        let new_word =
                            (&curr_word[0..j]).to_string() + &(k.to_string()) + &curr_word[j + 1..];
                        if new_word == end_word {
                            return distance + 1;
                        }

                        if !words.contains(&new_word) || new_word == curr_word {
                            continue;
                        }

                        if !reached.contains(&new_word) {
                            q.push(new_word.clone());
                            reached.insert(new_word);
                        }
                    }
                }
            }
            distance += 1;
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0127_example_1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec_string!["hot", "dot", "dog", "lot", "log", "cog"];
        let result = 5;

        assert_eq!(
            Solution::ladder_length(begin_word, end_word, word_list),
            result
        );
    }

    #[test]
    fn test_0127_example_2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec_string!["hot", "dot", "dog", "lot", "log"];
        let result = 0;

        assert_eq!(
            Solution::ladder_length(begin_word, end_word, word_list),
            result
        );
    }
}
