/**
 * [0792] Number of Matching Subsequences
 *
 * Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 *
 * 	For example, "ace" is a subsequence of "abcde".
 *
 *  
 * Example 1:
 *
 * Input: s = "abcde", words = ["a","bb","acd","ace"]
 * Output: 3
 * Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
 *
 * Example 2:
 *
 * Input: s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^4
 * 	1 <= words.length <= 5000
 * 	1 <= words[i].length <= 50
 * 	s and words[i] consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-matching-subsequences/
// discuss: https://leetcode.com/problems/number-of-matching-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-matching-subsequences/discuss/1290041/Rust-36ms-oneliner
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        words
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut m, word| {
                *m.entry(word).or_insert(0) += 1;
                m
            })
            .into_iter()
            .fold(0, |acc, (word, repeat)| {
                let mut ss = s.chars();
                acc + repeat
                    * word
                        .chars()
                        .map(|c| ss.find(|&s| s == c))
                        .last()
                        .unwrap()
                        .map_or(0, |_| 1)
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0792_example_1() {
        let s = "abcde".to_string();
        let words = vec_string!["a", "bb", "acd", "ace"];
        let result = 3;

        assert_eq!(Solution::num_matching_subseq(s, words), result);
    }

    #[test]
    fn test_0792_example_2() {
        let s = "dsahjpjauf".to_string();
        let words = vec_string!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
        let result = 2;

        assert_eq!(Solution::num_matching_subseq(s, words), result);
    }
}
