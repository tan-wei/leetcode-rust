/**
 * [49] Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 10^4
 * 	0 <= strs[i].length <= 100
 * 	strs[i] consists of lower-case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hashmap = HashMap::new();
        for v in strs {
            let mut k: Vec<_> = v.bytes().collect();
            k.sort_unstable();
            hashmap.entry(k).or_insert_with(|| vec![]).push(v)
        }
        hashmap.into_iter().map(|(_, v)| v).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0049_example_1() {
        let strs = vec_string!["eat", "tea", "tan", "ate", "nat", "bat"];
        let result = vec![
            vec_string!["bat"],
            vec_string!["nat", "tan"],
            vec_string!["ate", "eat", "tea"],
        ];

        assert_eq!(Solution::group_anagrams(strs), result);
    }

    #[test]
    #[ignore]
    fn test_0049_example_2() {
        let strs = vec_string![""];
        let result = vec![vec_string![""]];

        assert_eq!(Solution::group_anagrams(strs), result);
    }

    #[test]
    #[ignore]
    fn test_0049_example_3() {
        let strs = vec_string!["a"];
        let result = vec![vec_string!["a"]];

        assert_eq!(Solution::group_anagrams(strs), result);
    }
}
