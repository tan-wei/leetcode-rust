/**
 * [0522] Longest Uncommon Subsequence II
 *
 * Given an array of strings strs, return the length of the longest uncommon subsequence between them. If the longest uncommon subsequence does not exist, return -1.
 * An uncommon subsequence between an array of strings is a string that is a subsequence of one string but not the others.
 * A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.
 *
 * 	For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "a<u>e</u>b<u>d</u>c" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).
 *
 *  
 * Example 1:
 * Input: strs = ["aba","cdc","eae"]
 * Output: 3
 * Example 2:
 * Input: strs = ["aaa","aaa","aa"]
 * Output: -1
 *  
 * Constraints:
 *
 * 	2 <= strs.length <= 50
 * 	1 <= strs[i].length <= 10
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-uncommon-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/1428744/Rust-Solution
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut strs = strs;

        strs.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        (0..strs.len())
            .find(|&i| (0..strs.len()).all(|j| i == j || !Self::is_subsequence(&strs[i], &strs[j])))
            .map(|i| strs[i].len() as i32)
            .unwrap_or(-1)
    }

    fn is_subsequence(a: &str, b: &str) -> bool {
        let mut bcs = b.chars();
        a.chars().all(|ca| bcs.any(|cb| ca == cb))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0522_example_1() {
        let strs = vec_string!["aba", "cdc", "eae"];
        let result = 3;

        assert_eq!(Solution::find_lu_slength(strs), result);
    }

    #[test]
    fn test_0522_example_2() {
        let strs = vec_string!["aaa", "aaa", "aa"];
        let result = -1;

        assert_eq!(Solution::find_lu_slength(strs), result);
    }
}
