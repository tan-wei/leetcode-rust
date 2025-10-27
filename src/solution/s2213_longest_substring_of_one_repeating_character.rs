/**
 * [2213] Longest Substring of One Repeating Character
 *
 * You are given a 0-indexed string s. You are also given a 0-indexed string queryCharacters of length k and a 0-indexed array of integer indices queryIndices of length k, both of which are used to describe k queries.
 * The i^th query updates the character in s at index queryIndices[i] to the character queryCharacters[i].
 * Return an array lengths of length k where lengths[i] is the length of the longest substring of s consisting of only one repeating character after the i^th query is performed.
 *  
 * Example 1:
 *
 * Input: s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
 * Output: [3,3,4]
 * Explanation:
 * - 1^st query updates s = "<u>bbb</u>acc". The longest substring consisting of one repeating character is "bbb" with length 3.
 * - 2^nd query updates s = "bbb<u>ccc</u>".
 *   The longest substring consisting of one repeating character can be "bbb" or "ccc" with length 3.
 * - 3^rd query updates s = "<u>bbbb</u>cc". The longest substring consisting of one repeating character is "bbbb" with length 4.
 * Thus, we return [3,3,4].
 *
 * Example 2:
 *
 * Input: s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
 * Output: [2,3]
 * Explanation:
 * - 1^st query updates s = "aba<u>zz</u>". The longest substring consisting of one repeating character is "zz" with length 2.
 * - 2^nd query updates s = "<u>aaa</u>zz". The longest substring consisting of one repeating character is "aaa" with length 3.
 * Thus, we return [2,3].
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 * 	k == queryCharacters.length == queryIndices.length
 * 	1 <= k <= 10^5
 * 	queryCharacters consists of lowercase English letters.
 * 	0 <= queryIndices[i] < s.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-of-one-repeating-character/
// discuss: https://leetcode.com/problems/longest-substring-of-one-repeating-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2213_example_1() {
        let s = "babacc".to_string();
        let query_characters = "bcb".to_string();
        let query_indices = vec![1, 3, 3];

        let result = vec![3, 3, 4];

        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2213_example_2() {
        let s = "abyzz".to_string();
        let query_characters = "aa".to_string();
        let query_indices = vec![2, 1];

        let result = vec![2, 3];

        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            result
        );
    }
}
