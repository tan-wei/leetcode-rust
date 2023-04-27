/**
 * [1048] Longest String Chain
 *
 * You are given an array of words where each word consists of lowercase English letters.
 * wordA is a predecessor of wordB if and only if we can insert exactly one letter anywhere in wordA without changing the order of the other characters to make it equal to wordB.
 *
 * 	For example, "abc" is a predecessor of "ab<u>a</u>c", while "cba" is not a predecessor of "bcad".
 *
 * A word chain is a sequence of words [word1, word2, ..., wordk] with k >= 1, where word1 is a predecessor of word2, word2 is a predecessor of word3, and so on. A single word is trivially a word chain with k == 1.
 * Return the length of the longest possible word chain with words chosen from the given list of words.
 *  
 * Example 1:
 *
 * Input: words = ["a","b","ba","bca","bda","bdca"]
 * Output: 4
 * Explanation: One of the longest word chains is ["a","<u>b</u>a","b<u>d</u>a","bd<u>c</u>a"].
 *
 * Example 2:
 *
 * Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
 * Output: 5
 * Explanation: All the words can be put in a word chain ["xb", "xb<u>c</u>", "<u>c</u>xbc", "<u>p</u>cxbc", "pcxbc<u>f</u>"].
 *
 * Example 3:
 *
 * Input: words = ["abcd","dbqca"]
 * Output: 1
 * Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
 * ["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length <= 16
 * 	words[i] only consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-string-chain/
// discuss: https://leetcode.com/problems/longest-string-chain/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by_cached_key(String::len);
        let mut hm = std::collections::HashMap::new();
        let mut result = 0;
        for word in &words {
            let max = (0..word.len())
                .filter_map(|i| hm.get(&(String::new() + &word[0..i] + &word[i + 1..])))
                .max()
                .unwrap_or(&0)
                + 1;
            hm.insert(word, max);
            result = result.max(max);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1048_example_1() {
        let words = vec_string!["a", "b", "ba", "bca", "bda", "bdca"];
        let result = 4;

        assert_eq!(Solution::longest_str_chain(words), result);
    }

    #[test]
    fn test_1048_example_2() {
        let words = vec_string!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        let result = 5;

        assert_eq!(Solution::longest_str_chain(words), result);
    }

    #[test]
    fn test_1048_example_3() {
        let words = vec_string!["abcd", "dbqca"];
        let result = 1;

        assert_eq!(Solution::longest_str_chain(words), result);
    }
}
