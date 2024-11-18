/**
 * [1768] Merge Strings Alternately
 *
 * You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
 *
 * Return the merged string.
 *
 *  
 * Example 1:
 *
 *
 * Input: word1 = "abc", word2 = "pqr"
 * Output: "apbqcr"
 * Explanation: The merged string will be merged as so:
 * word1:  a   b   c
 * word2:    p   q   r
 * merged: a p b q c r
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "ab", word2 = "pqrs"
 * Output: "apbqrs"
 * Explanation: Notice that as word2 is longer, "rs" is appended to the end.
 * word1:  a   b
 * word2:    p   q   r   s
 * merged: a p b q   r   s
 *
 *
 * Example 3:
 *
 *
 * Input: word1 = "abcd", word2 = "pq"
 * Output: "apbqcd"
 * Explanation: Notice that as word1 is longer, "cd" is appended to the end.
 * word1:  a   b   c   d
 * word2:    p   q
 * merged: a p b q c   d
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= word1.length, word2.length <= 100
 * 	word1 and word2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-strings-alternately/
// discuss: https://leetcode.com/problems/merge-strings-alternately/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        let common_len = word1.len().min(word2.len());

        let zipped = std::iter::zip(
            chars1.by_ref().take(common_len),
            chars2.by_ref().take(common_len),
        );

        let mut result = String::with_capacity(word1.len() + word2.len());
        zipped.for_each(|(a, b)| {
            result.push(a);
            result.push(b)
        });

        result.extend(chars1);
        result.extend(chars2);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1768_example_1() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();

        let result = "apbqcr".to_string();

        assert_eq!(Solution::merge_alternately(word1, word2), result);
    }

    #[test]
    fn test_1768_example_2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();

        let result = "apbqrs".to_string();

        assert_eq!(Solution::merge_alternately(word1, word2), result);
    }

    #[test]
    fn test_1768_example_3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();

        let result = "apbqcd".to_string();

        assert_eq!(Solution::merge_alternately(word1, word2), result);
    }
}
