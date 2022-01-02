/**
 * [0472] Concatenated Words
 *
 * Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.
 * A concatenated word is defined as a string that is comprised entirely of at least two shorter words in the given array.
 *  
 * Example 1:
 *
 * Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
 * Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
 * Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats";
 * "dogcatsdog" can be concatenated by "dog", "cats" and "dog";
 * "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
 * Example 2:
 *
 * Input: words = ["cat","dog","catdog"]
 * Output: ["catdog"]
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 10^4
 * 	0 <= words[i].length <= 1000
 * 	words[i] consists of only lowercase English letters.
 * 	0 <= sum(words[i].length) <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenated-words/
// discuss: https://leetcode.com/problems/concatenated-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::hash::Hasher;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut hash_set = std::collections::HashSet::new();
        for word in &words {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            for byte in word.bytes() {
                hasher.write_u8(byte);
            }
            hash_set.insert(hasher.finish());
        }
        let mut res = Vec::new();
        for word in words {
            let s = word.bytes().collect::<Vec<u8>>();
            let n = s.len();
            if Self::dfs(0, 0, &hash_set, &s, n) {
                res.push(word);
            }
        }
        res
    }

    fn dfs(
        start: usize,
        k: usize,
        hash_set: &std::collections::HashSet<u64>,
        s: &[u8],
        n: usize,
    ) -> bool {
        if start == n {
            k >= 2
        } else {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            for i in start..n {
                hasher.write_u8(s[i]);
                if hash_set.contains(&hasher.finish()) && Self::dfs(i + 1, k + 1, hash_set, s, n) {
                    return true;
                }
            }
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0472_example_1() {
        let words = vec_string![
            "cat",
            "cats",
            "catsdogcats",
            "dog",
            "dogcatsdog",
            "hippopotamuses",
            "rat",
            "ratcatdogcat"
        ];

        let result = vec_string!["catsdogcats", "dogcatsdog", "ratcatdogcat"];

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(words),
            result
        );
    }

    #[test]
    fn test_0472_example_2() {
        let words = vec_string!["cat", "dog", "catdog"];

        let result = vec_string!["catdog"];

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(words),
            result
        );
    }
}
