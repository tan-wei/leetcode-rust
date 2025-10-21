/**
 * [2207] Maximize Number of Subsequences in a String
 *
 * You are given a 0-indexed string text and another 0-indexed string pattern of length 2, both of which consist of only lowercase English letters.
 * You can add either pattern[0] or pattern[1] anywhere in text exactly once. Note that the character can be added even at the beginning or at the end of text.
 * Return the maximum number of times pattern can occur as a subsequence of the modified text.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 *  
 * Example 1:
 *
 * Input: text = "abdcdbc", pattern = "ac"
 * Output: 4
 * Explanation:
 * If we add pattern[0] = 'a' in between text[1] and text[2], we get "ab<u>a</u>dcdbc". Now, the number of times "ac" occurs as a subsequence is 4.
 * Some other strings which have 4 subsequences "ac" after adding a character to text are "<u>a</u>abdcdbc" and "abd<u>a</u>cdbc".
 * However, strings such as "abdc<u>a</u>dbc", "abd<u>c</u>cdbc", and "abdcdbc<u>c</u>", although obtainable, have only 3 subsequences "ac" and are thus suboptimal.
 * It can be shown that it is not possible to get more than 4 subsequences "ac" by adding only one character.
 *
 * Example 2:
 *
 * Input: text = "aabb", pattern = "ab"
 * Output: 6
 * Explanation:
 * Some of the strings which can be obtained from text and have 6 subsequences "ab" are "<u>a</u>aabb", "aa<u>a</u>bb", and "aab<u>b</u>b".
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 10^5
 * 	pattern.length == 2
 * 	text and pattern consist only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-number-of-subsequences-in-a-string/
// discuss: https://leetcode.com/problems/maximize-number-of-subsequences-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2207_example_1() {
        let text = "abdcdbc".to_string();
        let pattern = "ac".to_string();

        let result = 4;

        assert_eq!(Solution::maximum_subsequence_count(text, pattern), result);
    }

    #[test]
    #[ignore]
    fn test_2207_example_2() {
        let text = "aabb".to_string();
        let pattern = "ab".to_string();

        let result = 6;

        assert_eq!(Solution::maximum_subsequence_count(text, pattern), result);
    }
}
