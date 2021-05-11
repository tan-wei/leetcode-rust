/**
 * [126] Word Ladder II
 *
 * A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
 *
 * 	Every adjacent pair of words differs by a single letter.
 * 	Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
 * 	sk == endWord
 *
 * Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest transformation sequences from beginWord to endWord, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].
 *  
 * Example 1:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
 * Output: [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
 * Explanation: There are 2 shortest transformation sequences:
 * "hit" -> "hot" -> "dot" -> "dog" -> "cog"
 * "hit" -> "hot" -> "lot" -> "log" -> "cog"
 *
 * Example 2:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
 * Output: []
 * Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
 *
 *  
 * Constraints:
 *
 * 	1 <= beginWord.length <= 5
 * 	endWord.length == beginWord.length
 * 	1 <= wordList.length <= 1000
 * 	wordList[i].length == beginWord.length
 * 	beginWord, endWord, and wordList[i] consist of lowercase English letters.
 * 	beginWord != endWord
 * 	All the words in wordList are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-ladder-ii/
// discuss: https://leetcode.com/problems/word-ladder-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/word-ladder-ii/discuss/803444/Rust-8ms-solution-double-ended-BFS-%2B-DFS
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::{HashMap, HashSet};
        use std::ops::Deref;

        if !word_list.contains(&end_word) {
            return vec![];
        };

        let mut word_warehouse = word_list.iter().map(String::deref).collect::<HashSet<_>>();
        let mut group_set_1 = HashSet::new();
        group_set_1.insert(begin_word.deref());
        let mut group_set_2 = HashSet::new();
        group_set_2.insert(end_word.deref());

        for word in group_set_1.iter() {
            word_warehouse.remove(word);
        }

        for word in group_set_2.iter() {
            word_warehouse.remove(word);
        }

        let mut word_link_map = HashMap::new();
        let mut found = false;
        let mut reversed = false;

        while !group_set_1.is_empty() && !group_set_2.is_empty() && !found {
            if group_set_1.len() > group_set_2.len() {
                std::mem::swap(&mut group_set_1, &mut group_set_2);
                reversed = !reversed;
            };

            let mut group_set_t: HashSet<&str> = HashSet::new();

            for &word1 in group_set_1.iter() {
                let mut curr = word1.to_owned();
                let bytes = unsafe { curr.as_bytes_mut() };
                for i in 0..bytes.len() {
                    let ch = bytes[i];
                    for ch in b'a'..=b'z' {
                        bytes[i] = ch;
                        let word_str = unsafe { std::str::from_utf8_unchecked(bytes) };
                        if let Some(word) = group_set_2.get(word_str).cloned() {
                            found = true;
                            let (from, to) = if reversed {
                                (word, word1)
                            } else {
                                (word1, word)
                            };
                            word_link_map.entry(from).or_insert(vec![]).push(to);
                        } else if let Some(word) = word_warehouse.get(word_str).cloned() {
                            if !found {
                                group_set_t.insert(word);
                                let (from, to) = if reversed {
                                    (word, word1)
                                } else {
                                    (word1, word)
                                };
                                word_link_map.entry(from).or_insert(vec![]).push(to);
                            };
                        };
                    }
                    bytes[i] = ch;
                }
            }

            for word in group_set_t.iter() {
                word_warehouse.remove(word);
            }
            std::mem::swap(&mut group_set_1, &mut group_set_t);
        }

        let mut stack = vec![];
        let mut results = vec![];

        Self::recursive_result(
            &word_link_map,
            &begin_word,
            &end_word,
            &mut stack,
            &mut results,
        );
        results
    }

    fn recursive_result(
        word_link_map: &std::collections::HashMap<&str, Vec<&str>>,
        from: &str,
        end: &str,
        stack: &mut Vec<String>,
        results: &mut Vec<Vec<String>>,
    ) {
        stack.push(from.to_owned());
        if from == end {
            results.push(stack.iter().cloned().collect());
        } else if let Some(inner) = word_link_map.get(from) {
            for word in inner.iter() {
                Self::recursive_result(word_link_map, word, end, stack, results);
            }
        };
        stack.pop();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0126_example_1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec_string!["hot", "dot", "dog", "lot", "log", "cog"];
        let result = vec![
            vec_string!["hit", "hot", "dot", "dog", "cog"],
            vec_string!["hit", "hot", "lot", "log", "cog"],
        ];

        assert_eq_sorted!(
            Solution::find_ladders(begin_word, end_word, word_list),
            result
        );
    }

    #[test]
    fn test_0126_example_2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec_string!["hot", "dot", "dog", "lot", "log"];
        let result: Vec<Vec<String>> = vec![];

        assert_eq_sorted!(
            Solution::find_ladders(begin_word, end_word, word_list),
            result
        );
    }
}
