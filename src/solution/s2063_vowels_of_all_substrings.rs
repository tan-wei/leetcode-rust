/**
 * [2063] Vowels of All Substrings
 *
 * Given a string word, return the sum of the number of vowels ('a', 'e', 'i', 'o', and 'u') in every substring of word.
 * A substring is a contiguous (non-empty) sequence of characters within a string.
 * Note: Due to the large constraints, the answer may not fit in a signed 32-bit integer. Please be careful during the calculations.
 *  
 * Example 1:
 *
 * Input: word = "aba"
 * Output: 6
 * Explanation:
 * All possible substrings are: "a", "ab", "aba", "b", "ba", and "a".
 * - "b" has 0 vowels in it
 * - "a", "ab", "ba", and "a" have 1 vowel each
 * - "aba" has 2 vowels in it
 * Hence, the total sum of vowels = 0 + 1 + 1 + 1 + 1 + 2 = 6.
 *
 * Example 2:
 *
 * Input: word = "abc"
 * Output: 3
 * Explanation:
 * All possible substrings are: "a", "ab", "abc", "b", "bc", and "c".
 * - "a", "ab", and "abc" have 1 vowel each
 * - "b", "bc", and "c" have 0 vowels each
 * Hence, the total sum of vowels = 1 + 1 + 1 + 0 + 0 + 0 = 3.
 *
 * Example 3:
 *
 * Input: word = "ltcd"
 * Output: 0
 * Explanation: There are no vowels in any substring of "ltcd".
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 10^5
 * 	word consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/vowels-of-all-substrings/
// discuss: https://leetcode.com/problems/vowels-of-all-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/vowels-of-all-substrings/solutions/5089523/functional-rust-solution/
    pub fn count_vowels(word: String) -> i64 {
        let n = word.len() as i64;
        word.chars()
            .enumerate()
            .filter(|(i, c)| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
            .map(|(i, c)| i as i64)
            .fold(0i64, |acc, i| acc + (i + 1) * (n - i))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2063_example_1() {
        let word = "aba".to_string();

        let result = 6;

        assert_eq!(Solution::count_vowels(word), result);
    }

    #[test]
    fn test_2063_example_2() {
        let word = "abc".to_string();

        let result = 3;

        assert_eq!(Solution::count_vowels(word), result);
    }

    #[test]
    fn test_2063_example_3() {
        let word = "ltcd".to_string();

        let result = 0;

        assert_eq!(Solution::count_vowels(word), result);
    }
}
