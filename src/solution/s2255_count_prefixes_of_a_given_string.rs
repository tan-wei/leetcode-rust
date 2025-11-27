/**
 * [2255] Count Prefixes of a Given String
 *
 * You are given a string array words and a string s, where words[i] and s comprise only of lowercase English letters.
 * Return the number of strings in words that are a prefix of s.
 * A prefix of a string is a substring that occurs at the beginning of the string. A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: words = ["a","b","c","ab","bc","abc"], s = "abc"
 * Output: 3
 * Explanation:
 * The strings in words which are a prefix of s = "abc" are:
 * "a", "ab", and "abc".
 * Thus the number of strings in words which are a prefix of s is 3.
 * Example 2:
 *
 * Input: words = ["a","a"], s = "aa"
 * Output: 2
 * Explanation:
 * Both of the strings are a prefix of s.
 * Note that the same string can occur multiple times in words, and it should be counted each time.
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length, s.length <= 10
 * 	words[i] and s consist of lowercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-prefixes-of-a-given-string/
// discuss: https://leetcode.com/problems/count-prefixes-of-a-given-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2255_example_1() {
        let words = vec_string!["a", "b", "c", "ab", "bc", "abc"];
        let s = "abc".to_string();

        let result = 3;

        assert_eq!(Solution::count_prefixes(words, s), result);
    }

    #[test]
    #[ignore]
    fn test_2255_example_2() {
        let words = vec_string!["a", "a"];
        let s = "aa".to_string();

        let result = 2;

        assert_eq!(Solution::count_prefixes(words, s), result);
    }
}
