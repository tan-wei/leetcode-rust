/**
 * [1170] Compare Strings by Frequency of the Smallest Character
 *
 * Let the function f(s) be the frequency of the lexicographically smallest character in a non-empty string s. For example, if s = "dcce" then f(s) = 2 because the lexicographically smallest character is 'c', which has a frequency of 2.
 * You are given an array of strings words and another array of query strings queries. For each query queries[i], count the number of words in words such that f(queries[i]) < f(W) for each W in words.
 * Return an integer array answer, where each answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 *
 * Input: queries = ["cbd"], words = ["zaaaz"]
 * Output: [1]
 * Explanation: On the first query we have f("cbd") = 1, f("zaaaz") = 3 so f("cbd") < f("zaaaz").
 *
 * Example 2:
 *
 * Input: queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
 * Output: [1,2]
 * Explanation: On the first query only f("bbb") < f("aaaa"). On the second query both f("aaa") and f("aaaa") are both > f("cc").
 *
 *  
 * Constraints:
 *
 * 	1 <= queries.length <= 2000
 * 	1 <= words.length <= 2000
 * 	1 <= queries[i].length, words[i].length <= 10
 * 	queries[i][j], words[i][j] consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character/
// discuss: https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut word_freqency = [0; 12];

        words
            .into_iter()
            .for_each(|word| word_freqency[word.matches(word.chars().min().unwrap()).count()] += 1);
        for i in (0..10).rev() {
            word_freqency[i] += word_freqency[i + 1];
        }
        queries
            .into_iter()
            .map(|word| word_freqency[word.matches(word.chars().min().unwrap()).count() + 1])
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1170_example_1() {
        let queries = vec_string!["cbd"];
        let words = vec_string!["zaaaz"];
        let result = vec![1];

        assert_eq!(Solution::num_smaller_by_frequency(queries, words), result);
    }

    #[test]
    fn test_1170_example_2() {
        let queries = vec_string!["bbb", "cc"];
        let words = vec_string!["a", "aa", "aaa", "aaaa"];
        let result = vec![1, 2];

        assert_eq!(Solution::num_smaller_by_frequency(queries, words), result);
    }
}
