/**
 * [2068] Check Whether Two Strings are Almost Equivalent
 *
 * Two strings word1 and word2 are considered almost equivalent if the differences between the frequencies of each letter from 'a' to 'z' between word1 and word2 is at most 3.
 * Given two strings word1 and word2, each of length n, return true if word1 and word2 are almost equivalent, or false otherwise.
 * The frequency of a letter x is the number of times it occurs in the string.
 *  
 * Example 1:
 *
 * Input: word1 = "aaaa", word2 = "bccb"
 * Output: false
 * Explanation: There are 4 'a's in "aaaa" but 0 'a's in "bccb".
 * The difference is 4, which is more than the allowed 3.
 *
 * Example 2:
 *
 * Input: word1 = "abcdeef", word2 = "abaaacc"
 * Output: true
 * Explanation: The differences between the frequencies of each letter in word1 and word2 are at most 3:
 * - 'a' appears 1 time in word1 and 4 times in word2. The difference is 3.
 * - 'b' appears 1 time in word1 and 1 time in word2. The difference is 0.
 * - 'c' appears 1 time in word1 and 2 times in word2. The difference is 1.
 * - 'd' appears 1 time in word1 and 0 times in word2. The difference is 1.
 * - 'e' appears 2 times in word1 and 0 times in word2. The difference is 2.
 * - 'f' appears 1 time in word1 and 0 times in word2. The difference is 1.
 *
 * Example 3:
 *
 * Input: word1 = "cccddabba", word2 = "babababab"
 * Output: true
 * Explanation: The differences between the frequencies of each letter in word1 and word2 are at most 3:
 * - 'a' appears 2 times in word1 and 4 times in word2. The difference is 2.
 * - 'b' appears 2 times in word1 and 5 times in word2. The difference is 3.
 * - 'c' appears 3 times in word1 and 0 times in word2. The difference is 3.
 * - 'd' appears 2 times in word1 and 0 times in word2. The difference is 2.
 *
 *  
 * Constraints:
 *
 * 	n == word1.length == word2.length
 * 	1 <= n <= 100
 * 	word1 and word2 consist only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/
// discuss: https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut dict = [0; 26];

        for b in word1.as_bytes() {
            dict[(*b - b'a') as usize] += 1;
        }

        for b in word2.as_bytes() {
            dict[(*b - b'a') as usize] -= 1;
        }
        for v in dict {
            if v > 3 || v < -3 {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2068_example_1() {
        let word1 = "aaaa".to_string();
        let word2 = "bccb".to_string();

        let result = false;

        assert_eq!(Solution::check_almost_equivalent(word1, word2), result);
    }

    #[test]
    fn test_2068_example_2() {
        let word1 = "abcdeef".to_string();
        let word2 = "abaaacc".to_string();

        let result = true;

        assert_eq!(Solution::check_almost_equivalent(word1, word2), result);
    }

    #[test]
    fn test_2068_example_3() {
        let word1 = "cccddabba".to_string();
        let word2 = "babababab".to_string();

        let result = true;

        assert_eq!(Solution::check_almost_equivalent(word1, word2), result);
    }
}
