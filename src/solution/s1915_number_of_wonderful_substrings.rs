/**
 * [1915] Number of Wonderful Substrings
 *
 * A wonderful string is a string where at most one letter appears an odd number of times.
 *
 *
 * 	For example, "ccjjc" and "abab" are wonderful, but "ab" is not.
 *
 *
 * Given a string word that consists of the first ten lowercase English letters ('a' through 'j'), return the number of wonderful non-empty substrings in word. If the same substring appears multiple times in word, then count each occurrence separately.
 *
 * A substring is a contiguous sequence of characters in a string.
 *
 *  
 * Example 1:
 *
 *
 * Input: word = "aba"
 * Output: 4
 * Explanation: The four wonderful substrings are underlined below:
 * - "<u>a</u>ba" -> "a"
 * - "a<u>b</u>a" -> "b"
 * - "ab<u>a</u>" -> "a"
 * - "<u>aba</u>" -> "aba"
 *
 *
 * Example 2:
 *
 *
 * Input: word = "aabb"
 * Output: 9
 * Explanation: The nine wonderful substrings are underlined below:
 * - "<u>a</u>abb" -> "a"
 * - "<u>aa</u>bb" -> "aa"
 * - "<u>aab</u>b" -> "aab"
 * - "<u>aabb</u>" -> "aabb"
 * - "a<u>a</u>bb" -> "a"
 * - "a<u>abb</u>" -> "abb"
 * - "aa<u>b</u>b" -> "b"
 * - "aa<u>bb</u>" -> "bb"
 * - "aab<u>b</u>" -> "b"
 *
 *
 * Example 3:
 *
 *
 * Input: word = "he"
 * Output: 2
 * Explanation: The two wonderful substrings are underlined below:
 * - "<u>h</u>e" -> "h"
 * - "h<u>e</u>" -> "e"
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= word.length <= 10^5
 * 	word consists of lowercase English letters from 'a' to 'j'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-wonderful-substrings/
// discuss: https://leetcode.com/problems/number-of-wonderful-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        word.bytes()
            .fold(
                (0usize, 0i64, {
                    let mut v = vec![0i64; 1024];
                    v[0] = 1;
                    v
                }),
                |(mut x, mut ans, mut v), c| {
                    x ^= 1 << (c - 97);
                    ans += v[x];
                    ans += (0..10).map(|i| v[x ^ (1 << i)]).sum::<i64>();
                    v[x] += 1;
                    (x, ans, v)
                },
            )
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1915_example_1() {
        let word = "aba".to_string();

        let result = 4;

        assert_eq!(Solution::wonderful_substrings(word), result);
    }

    #[test]
    fn test_1915_example_2() {
        let word = "aabb".to_string();

        let result = 9;

        assert_eq!(Solution::wonderful_substrings(word), result);
    }

    #[test]
    fn test_1915_example_3() {
        let word = "he".to_string();

        let result = 2;

        assert_eq!(Solution::wonderful_substrings(word), result);
    }
}
