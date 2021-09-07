/**
 * [318] Maximum Product of Word Lengths
 *
 * Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.
 *  
 * Example 1:
 *
 * Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
 * Output: 16
 * Explanation: The two words can be "abcw", "xtfn".
 *
 * Example 2:
 *
 * Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
 * Output: 4
 * Explanation: The two words can be "ab", "cd".
 *
 * Example 3:
 *
 * Input: words = ["a","aa","aaa","aaaa"]
 * Output: 0
 * Explanation: No such pair of words.
 *
 *  
 * Constraints:
 *
 * 	2 <= words.length <= 1000
 * 	1 <= words[i].length <= 1000
 * 	words[i] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-word-lengths/
// discuss: https://leetcode.com/problems/maximum-product-of-word-lengths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let len = words.len();
        let bit_masks: Vec<u32> = {
            let mut bit_masks = vec![0; len];

            for (idx, word) in words.iter().enumerate() {
                for ch in word.chars() {
                    bit_masks[idx] |= 1 << (ch as u32 - 'a' as u32);
                }
            }
            bit_masks
        };

        let mut result: i32 = 0;
        for lo in 0..len {
            for hi in lo + 1..len {
                if bit_masks[lo] & bit_masks[hi] == 0 {
                    result =
                        std::cmp::max(result, (words[lo].len() as i32) * (words[hi].len() as i32));
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0318_example_1() {
        let words = vec_string!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        let result = 16;

        assert_eq!(Solution::max_product(words), result);
    }

    #[test]
    fn test_0318_example_2() {
        let words = vec_string!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
        let result = 4;

        assert_eq!(Solution::max_product(words), result);
    }

    #[test]
    fn test_0318_example_3() {
        let words = vec_string!["a", "aa", "aaa", "aaaa"];
        let result = 0;

        assert_eq!(Solution::max_product(words), result);
    }
}
